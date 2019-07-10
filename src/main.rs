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



#[post("/play/victory")]
fn play_victory() {
    let device = rodio::default_output_device().unwrap();
    let file = std::fs::File::open("examples/victory.mp3").unwrap();
    let victory = rodio::play_once(&device, BufReader::new(file)).unwrap();
    victory.set_volume(1.0);
    println!("VICTORY\n");
    NamedFile::open("static/index.html");
    thread::sleep(Duration::from_millis(4500));
}

#[post("/play/jiggly")]
fn play_jiggly() {
    let device = rodio::default_output_device().unwrap();
    let file = std::fs::File::open("examples/jiggly.mp3").unwrap();
    let victory = rodio::play_once(&device, BufReader::new(file)).unwrap();
    victory.set_volume(1.0);
    println!("JIGGLY\n");
    thread::sleep(Duration::from_millis(8000));
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, play_victory, play_jiggly])
}

fn main() {
    rocket().launch();
}
