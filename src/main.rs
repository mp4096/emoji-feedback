#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;

mod file_utils;

use chrono::Local;
use rocket::response::NamedFile;
use rocket::State;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicUsize;

struct Timings {
    timestamp: AtomicUsize,
}
struct Config {
    log_file: PathBuf,
    cooldown: usize, // cooldown in seconds
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/feedback/<fb>")]
fn feedback(fb: &str, last: State<Timings>, conf: State<Config>) -> Result<(), io::Error> {
    use file_utils::append_line_to_file;
    use std::io::ErrorKind;
    use std::sync::atomic::Ordering;

    let curr_timestamp = Local::now().timestamp() as usize;
    let time_since_last = curr_timestamp - last.timestamp.load(Ordering::Relaxed);
    last.timestamp.store(curr_timestamp, Ordering::Relaxed);

    if time_since_last < conf.cooldown {
        return Err(io::Error::new(ErrorKind::Other, "button mashing detected"));
    }

    let fb_int = match fb {
        "bad" => Ok(-1),
        "neutral" => Ok(0),
        "good" => Ok(1),
        _ => Err(io::Error::new(ErrorKind::InvalidInput, "invalid feedback value")),
    }?;

    let line = format!("{},{:>2}", Local::now().to_rfc3339(), fb_int);
    append_line_to_file(&conf.log_file, line)?;

    Ok(())
}

fn main() {
    let conf = Config {
        log_file: PathBuf::from("./feedback.csv"),
        cooldown: 1, // seconds
    };
    let tmgs = Timings { timestamp: AtomicUsize::new(Local::now().timestamp() as usize) };

    rocket::ignite()
        .mount("/", routes![index, files, feedback])
        .manage(tmgs)
        .manage(conf)
        .launch();
}
