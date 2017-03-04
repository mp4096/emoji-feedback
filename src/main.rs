#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/feedback/<fb>")]
fn feedback(fb: &str) {
    println!("{:?}", fb);
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files, feedback])
}

fn main() {
    rocket().launch();
}
