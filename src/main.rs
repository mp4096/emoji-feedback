#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::io;
use std::path::{Path, PathBuf};

use rocket_contrib::JSON;
use rocket::response::NamedFile;

#[derive(Debug, Deserialize)]
struct Feedback {
    feedback_value: usize,
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/feedback", format = "application/json", data = "<val>")]
fn feedback(val: JSON<Feedback>) {
    println!("{:?}", val);
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files, feedback])
}

fn main() {
    rocket().launch();
}
