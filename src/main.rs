#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;

use rocket::Rocket;
use rocket::request::{FlashMessage};
use rocket_contrib::{templates::Template, serve::StaticFiles};


use std::io::BufReader;
use std::thread;
use std::time::Duration;


#[get("/v")]
fn play_victory() {
    let device = rodio::default_output_device().unwrap();
    let file = std::fs::File::open("media/victory.mp3").unwrap();
    let victory = rodio::play_once(&device, BufReader::new(file)).unwrap();
    victory.set_volume(1.0);

    // Example metadata I/O
    let tag = id3::Tag::read_from_path("media/victory.mp3").unwrap();
    println!("{} {}", tag.title().unwrap() , tag.album().unwrap());


    thread::sleep(Duration::from_millis(4500));
}


#[derive(Debug, Serialize)]
struct Context<'a, 'b>{ msg: Option<(&'a str, &'b str)> }

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(msg: &'a str) -> Context<'static, 'a> {
        Context{msg: Some(("error", msg))}
    }

    pub fn raw(msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{msg: msg}
    }
}


/*
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
*/

#[get("/")]
fn index(msg: Option<FlashMessage<'_, '_>>) -> Template {
    Template::render("index", &match msg {
        Some(ref msg) => Context::raw(Some((msg.name(), msg.msg()))),
        None => Context::raw(None),
    })
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("static/"))
	.mount("/", routes![index, play_victory])
        .attach(Template::fairing())
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
