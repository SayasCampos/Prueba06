#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate mime;

use std::io::BufReader;
use std::thread;
use std::time::Duration;

use std::fs::File;
use std::io::prelude::*;


use std::io;
use rocket::request::{Form, FormError, FormDataError};
use rocket::response::NamedFile;
use rocket::http::RawStr;



#[get("/v", rank=2)]
fn play_victory() {
    let device = rodio::default_output_device().unwrap();
    let file = std::fs::File::open("media/victory.mp3").unwrap();
    let victory = rodio::play_once(&device, BufReader::new(file)).unwrap();
    victory.set_volume(1.0);

    // Example metadata I/O
    let tag = id3::Tag::read_from_path("media/victory.mp3").unwrap();
    println!("{} {}", tag.title().unwrap() , tag.album().unwrap());

    


    NamedFile::open("static/index.html");
    thread::sleep(Duration::from_millis(4500));
}

#[get("/j", rank=1)]
fn play_jiggly() {
    let device = rodio::default_output_device().unwrap();
    let file = std::fs::File::open("media/jiggly.mp3").unwrap();
    let victory = rodio::play_once(&device, BufReader::new(file)).unwrap();
    victory.set_volume(1.0);
    println!("JIGGLY\n");
    thread::sleep(Duration::from_millis(8000));
}

#[get("/", rank=3)]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, play_victory, play_jiggly])
}

fn main() {

    // Example playlist entry
    let playlist = vec![
        m3u::path_entry(r"Alternative\Band - Song.mp3"),
        m3u::path_entry(r"Classical\Other Band - New Song.mp3"),
        m3u::path_entry(r"Stuff.mp3"),
        m3u::path_entry(r"D:\More Music\Foo.mp3"),
        m3u::path_entry(r"..\Other Music\Bar.mp3"),
        m3u::url_entry(r"http://emp.cx:8000/Listen.pls").unwrap(),
        m3u::url_entry(r"http://www.example.com/~user/Mine.mp3").unwrap(),
    ];
    println!("There are {} items in current playlist", playlist.len());


    // Example playlist I/O
    let mut reader = m3u::Reader::open("media/playlist.m3u").unwrap();
    let read_playlist: Vec<_> = reader.entries().map(|entry| entry.unwrap()).collect();
    println!("Uploaded {} tracks to a playlist", read_playlist.len());
    
    rocket().launch();
}
