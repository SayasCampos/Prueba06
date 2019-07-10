#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate mime;
extern crate rodio;

use std::io::BufReader;
use std::thread;
use std::time::Duration;

use std::fs::File;
use std::io::prelude::*;


use std::io;
use rocket::request::{Form, FormError, FormDataError};
use rocket::response::NamedFile;
use rocket::http::RawStr;


/*

    let device = rodio::default_output_device().unwrap();
    let file = std::fs::File::open("victory.mp3").unwrap();
    let victory = rodio::play_once(&device, BufReader::new(file)).unwrap();
    victory.set_volume(1.0);
    println!("victory!");

#[derive(Debug, FromForm)]
struct FormInput {
    a: String,
    b: bool,
}


#[post("/", data = "<play>")]
fn play(play: Result<Form<FormInput>, FormError>) -> String {
    match play {
        Ok(form) => format!("{:?}", &*form),
        Err(FormDataError::Io(_)) => format!("Form input was invalid UTF-8"),
        Err(FormDataError::Malformed(f)) | Err(FormDataError::Parse(_, f)) => {
            format!("Invalid form input: {}", f)
        }
    }
}
*/

#[post("/play")]
fn play() {
    let device = rodio::default_output_device().unwrap();
    let file = std::fs::File::open("examples/victory.mp3").unwrap();
    let victory = rodio::play_once(&device, BufReader::new(file)).unwrap();
    victory.set_volume(100.0);
    println!("VICTORY\n");
}


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, play])
}

fn main() {
    rocket().launch();
}
