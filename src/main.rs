#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;

use rocket::request::FlashMessage;
use rocket::Rocket;
use rocket_contrib::{serve::StaticFiles, templates::Template};

use std::io::BufReader;
use std::thread;
use std::time::Duration;
use std::path::{Path, PathBuf}; // for I/O

use id3::frame::{Picture, PictureType}; // for album cover
use id3_image::extract_first_image; // for album cover

//MOVED DEFINITION TO EXTERNAL FILE track.rs - max
mod mapgen;
use mapgen::get_map;
use mapgen::track::Track;

#[post("/")]
fn play_victory() -> String {
    let device = rodio::default_output_device().unwrap();

//    let _current_song: Track = Track::new("media/victory.mp3".to_string());
    let _current_song_path = Path::new("media/victory.mp3");
    let _current_song: Track = Track::new(_current_song_path);
    let file = std::fs::File::open("media/victory.mp3").unwrap();
    let victory = rodio::play_once(&device, BufReader::new(file)).unwrap();
    victory.set_volume(1.0);

    println!("{}", _current_song.title);

    // Sandboxing around to import album art
    /*
    let albumArt = Picture {
        mime_type: PictureType::Other,
        description: String::new(),
        data: Vec::new(),
    };
    */
    
    thread::sleep(Duration::from_millis(4500));
    "success".to_string()
}

#[derive(Debug, Serialize)]
struct Context<'a, 'b> {
    msg: Option<(&'a str, &'b str)>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(msg: &'a str) -> Context<'static, 'a> {
        Context {
            msg: Some(("error", msg)),
        }
    }

    pub fn raw(msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context { msg: msg }
    }
}

#[get("/")]
fn index(msg: Option<FlashMessage<'_, '_>>) -> Template {
    Template::render(
        "index",
        &match msg {
            Some(ref msg) => Context::raw(Some((msg.name(), msg.msg()))),
            None => Context::raw(None),
        },
    )
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index, play_victory])
        .attach(Template::fairing())
}

fn main() {
    // Example playlist entry

    let media_dir = Path::new("media/");
    let music_lib = get_map(&media_dir);

    match music_lib {
        Ok(a) => {
            println!("HashMap has {} values\n", a.len());
            for b in a.keys() {
                let track = a.get(b).unwrap();
                println!("{}", track.title);
            }
        },
        Err(_) => println!("ERROR READING MUSIC LIBRARY"),
    }
    


    // Example playlist I/O
    let mut reader = m3u::Reader::open("media/playlist.m3u").unwrap();
    let read_playlist: Vec<_> = reader.entries().map(|entry| entry.unwrap()).collect();
    println!("Uploaded {} tracks to a playlist", read_playlist.len());

    rocket().launch();
}
