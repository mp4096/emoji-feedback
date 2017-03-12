#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use]
extern crate clap;
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate toml;

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

#[derive(Deserialize)]
struct Config {
    log_file: PathBuf,
    cooldown_ms: usize, // cooldown in milliseconds
    template: Template,
}

#[derive(Deserialize)]
struct Template {
    title: String,
    question: String,
}

fn load_config_file<T>(path: T) -> Result<Config, io::Error>
    where T: AsRef<Path>
{
    use file_utils::read_file;
    use std::io::ErrorKind;

    let toml_str = read_file(path.as_ref())?;
    toml::from_str(&toml_str)
        .map_err(|_| io::Error::new(ErrorKind::InvalidInput, "Could not parse TOML file"))
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

    if ms_since_last < conf.cooldown_ms {
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
    use clap::{App, Arg};
    use std::process;

    let m = App::new("emoji-feedback")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Emoji feedback server")
        .arg(Arg::with_name("config_file_path")
            .help("Path to the config file")
            .index(1)
            .required(true))
        .get_matches();

    let config_file_path = Path::new(m.value_of("config_file_path").unwrap());

    match load_config_file(&config_file_path) {
        Ok(conf) => {
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
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}
