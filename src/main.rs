#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;

mod file_utils;

use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/feedback/<fb>")]
fn feedback(fb: &str) -> Result<(), io::Error> {
    use chrono::{DateTime, Local};
    use file_utils::append_line_to_file;
    use std::io::ErrorKind;

    let timestamp: DateTime<Local> = Local::now();
    let fb_int = match fb {
        "bad" => Ok(-1),
        "neutral" => Ok(0),
        "good" => Ok(1),
        _ => Err(io::Error::new(ErrorKind::InvalidInput, "invalid feedback value")),
    }?;

    let line = format!("{},{:>2}", timestamp.to_rfc3339(), fb_int);
    append_line_to_file(&Path::new("./feedback.csv"), line)?;

    Ok(())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files, feedback])
}

fn main() {
    rocket().launch();
}
