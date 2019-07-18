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

use id3::frame::{Picture, PictureType}; // for album cover
use id3_image::extract_first_image;     // for album cover

pub struct Song {
    pub title:  String,
    pub album:  String,
    pub artist: String,
    pub genre:  String,
    pub year:   u32,
    pub track:  u32,
    //albumArt
}

impl Song {
    pub fn new(name: String) -> Song {
        Song {
            title:  "Unknown".to_string(),
            album:  "Unknown".to_string(),
            artist: "Unknown".to_string(),
            genre: "Unknown".to_string(),
            year: 0,
            track: 0,
        }
    }
}


#[post("/")]
fn play_victory() {
    let device = rodio::default_output_device().unwrap();
    let file = std::fs::File::open("media/victory.mp3").unwrap();
    let victory = rodio::play_once(&device, BufReader::new(file)).unwrap();
    victory.set_volume(1.0);

    // Example metadata I/O
    let tag = id3::Tag::read_from_path("media/victory.mp3").unwrap();
    println!("{} {}", tag.title().unwrap() , tag.album().unwrap());

    
    let currentSong = Song{
        title: tag.title().unwrap().to_string(),
        album: "".to_string(),
        artist: "".to_string(),
        genre: "".to_string(),
        year: 0,
        track: 0,
    };
        /*
        match tag.tite().unwrap().to_string().len() {
            0 => "Unknown".to_string(), 
            _ => title: tag.title().unwrap().to_string(),
        }
        */
    /*
        title: tag.title().unwrap().to_string(),
        album: "".to_string(),
        artist: "".to_string()
        //album: tag.album().unwrap().to_string(),
        //artist: tag.artist().unwrap().to_string()
    */


    //println!("{}", currentSong.title);

    /*
    let albumArt = Picture {
        mime_type: PictureType::Other,
        description: String::new(),
        data: Vec::new(),
    };
    */

    /*
    let mut music_filename
    let albumArt = extract_first_image(music_filename, image_filename);
    */


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
