#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;

mod file_utils;

use chrono::{DateTime, Local};
use rocket::response::NamedFile;
use rocket::State;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicUsize;

struct Timings {
    launch_time: DateTime<Local>,
    last_ms_from_launch: AtomicUsize, // milliseconds from the launch time
}
struct Config {
    log_file: PathBuf,
    cooldown: usize, // cooldown in milliseconds
}

#[inline]
fn count_ms_from_datetime(dt0: DateTime<Local>) -> usize {
    Local::now().signed_duration_since(dt0).num_milliseconds() as usize
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
fn feedback(fb: &str, tmgs: State<Timings>, conf: State<Config>) -> Result<(), io::Error> {
    use file_utils::append_line_to_file;
    use std::io::ErrorKind;
    use std::sync::atomic::Ordering;

    let curr_timestamp = count_ms_from_datetime(tmgs.launch_time);
    let ms_since_last = curr_timestamp - tmgs.last_ms_from_launch.load(Ordering::Relaxed);
    tmgs.last_ms_from_launch.store(curr_timestamp, Ordering::Relaxed);

    if ms_since_last < conf.cooldown {
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
        cooldown: 500, // milliseconds
    };
    let tmgs = Timings {
        launch_time: Local::now(),
        last_ms_from_launch: AtomicUsize::new(0),
    };

    rocket::ignite()
        .mount("/", routes![index, files, feedback])
        .manage(tmgs)
        .manage(conf)
        .launch();
}
