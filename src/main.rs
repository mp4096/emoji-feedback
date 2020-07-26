#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod auth_utils;
mod file_utils;

use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::templates::Template;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicUsize;

struct Timings {
    launch_time: chrono::DateTime<chrono::Local>,
    last_ms_from_launch: AtomicUsize, // milliseconds from the launch time
}

#[derive(Deserialize)]
struct Config {
    log_file: PathBuf,
    backup_file: PathBuf,
    cooldown_ms: usize, // cooldown in milliseconds
    template_context: TemplateContext,
    auth: AuthData,
}

#[derive(Deserialize)]
struct AuthData {
    salt: String,
    hash: String,
}

#[derive(Serialize, Deserialize)]
struct TemplateContext {
    title: String,
    question: String,
    thanks: String,
    acks: String,
}

fn load_config_file<T>(path: T) -> Result<Config, io::Error>
where
    T: AsRef<Path>,
{
    use file_utils::read_file;
    use std::io::ErrorKind;

    let toml_str = read_file(path.as_ref())?;
    toml::from_str(&toml_str)
        .map_err(|_| io::Error::new(ErrorKind::InvalidInput, "Could not parse TOML file"))
}

#[inline]
fn count_ms_from_datetime(dt0: chrono::DateTime<chrono::Local>) -> usize {
    chrono::Local::now()
        .signed_duration_since(dt0)
        .num_milliseconds() as usize
}

#[get("/")]
fn index(conf: State<Config>) -> Template {
    Template::render("index", &conf.template_context)
}

#[get("/log_file/<token>")]
fn serve_log_file(token: String, conf: State<Config>) -> Result<NamedFile, io::Error> {
    use auth_utils::check_access_token;
    use std::io::ErrorKind;

    // What can happen here:
    // * salt is invalid base64
    // * access token is invalid base64
    // * hash value in the config file is invalid base64
    // * there is no log file (yet)
    // * access token is unacceptable
    // All these cases result in the same 404 response.
    //
    // I warned you that this program is quick and dirty!
    if check_access_token(&token, &conf.auth.salt, &conf.auth.hash) {
        NamedFile::open(&conf.log_file)
    } else {
        Err(io::Error::new(
            ErrorKind::PermissionDenied,
            "access token invalid",
        ))
    }
}

#[delete("/log_file/<token>")]
fn reset_log_file(token: String, conf: State<Config>) -> Result<(), io::Error> {
    use auth_utils::check_access_token;
    use std::fs;
    use std::io::ErrorKind;

    if check_access_token(&token, &conf.auth.salt, &conf.auth.hash) {
        fs::rename(&conf.log_file, &conf.backup_file)
    } else {
        Err(io::Error::new(
            ErrorKind::PermissionDenied,
            "access token invalid",
        ))
    }
}

#[get("/public/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/feedback/<fb>")]
fn feedback(fb: String, tmgs: State<Timings>, conf: State<Config>) -> Result<(), io::Error> {
    use file_utils::append_line_to_file;
    use std::io::ErrorKind;
    use std::sync::atomic::Ordering;

    let curr_timestamp = count_ms_from_datetime(tmgs.launch_time);
    let ms_since_last = curr_timestamp - tmgs.last_ms_from_launch.load(Ordering::Relaxed);
    tmgs.last_ms_from_launch
        .store(curr_timestamp, Ordering::Relaxed);

    if ms_since_last < conf.cooldown_ms {
        return Err(io::Error::new(ErrorKind::Other, "button mashing detected"));
    }

    let fb_int = match fb.as_ref() {
        "so_very_negative" => Ok(-3),
        "very_negative" => Ok(-2),
        "negative" => Ok(-1),
        "neutral" => Ok(0),
        "positive" => Ok(1),
        "very_positive" => Ok(2),
        "so_very_positive" => Ok(3), // Yes! So. Very. Positive.
        _ => Err(io::Error::new(
            ErrorKind::InvalidInput,
            "invalid feedback value",
        )),
    }?;

    let line = format!("{},{:>2}", chrono::Local::now().to_rfc3339(), fb_int);
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
        .arg(
            Arg::with_name("config_file_path")
                .help("Path to the config file")
                .index(1)
                .required(true),
        )
        .get_matches();

    let config_file_path = Path::new(m.value_of("config_file_path").unwrap());

    match load_config_file(&config_file_path) {
        Ok(conf) => {
            let tmgs = Timings {
                launch_time: chrono::Local::now(),
                last_ms_from_launch: AtomicUsize::new(0),
            };

            rocket::ignite()
                .mount(
                    "/",
                    routes![index, files, serve_log_file, reset_log_file, feedback],
                )
                .manage(tmgs)
                .manage(conf)
                .attach(Template::fairing())
                .launch();
        }
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}
