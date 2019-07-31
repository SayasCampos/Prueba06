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
use std::path::Path;
use std::thread;
use std::time::Duration;

//MOVED DEFINITION TO EXTERNAL FILE track.rs - max
mod mapgen;
use mapgen::get_map;
use mapgen::track::Track;
use rocket_contrib::json::Json;
use rodio::Source;
use std::cell::RefCell;

//////////////////basis for the wrapped code found here
//////////////////https://stackoverflow.com/questions/19605132/is-it-possible-to-use-global-variables-in-rust

thread_local!(static DEVICE: RefCell<rodio::Device> = RefCell::new(rodio::default_output_device().unwrap()));
thread_local!(static SINK: RefCell<rodio::Sink> = RefCell::new(rodio::Sink::new(&rodio::default_output_device().unwrap())));
/////////////////end wrapped code

#[derive(Serialize, Deserialize)]
struct MyTrack {
    track_list: Vec<Track> 
}

fn get_track_list() -> Vec<mapgen::track::Track> {
    let media_dir = Path::new("media/");
    let music_lib = get_map(&media_dir);
    let mut track_vec: Vec<Track> = Vec::new();

    match music_lib {
        Ok(a) => {
            for b in a.keys() {
                let track = a.get(b).unwrap();
                track_vec.push(track.clone());
            }
        }
        Err(_) => println!("ERROR READING MUSIC LIBRARY"),
    }

    track_vec
}

fn get_track(track_name: String) -> mapgen::track::Track {
    let media_dir = Path::new("media/");
    let music_lib = get_map(&media_dir);
    let mut new_track: Track = Track::new("/media");
        
    match music_lib {
        Ok(a) => {
            for b in a.keys() {
                let track = a.get(b).unwrap();
                if track.title == track_name {
                    new_track = track.clone();
                }
            }
        }
        Err(_) => println!("ERROR READING MUSIC LIBRARY"),
    }

    new_track
}

#[post("/pause")]
fn pause(){
        SINK.with(|sink_cell| {
            let sink = sink_cell.borrow_mut();
            sink.pause();
        });
}

#[post("/stop")]
fn stop(){
        SINK.with(|sink_cell| {
            let sink = sink_cell.borrow_mut();
            sink.stop();
        });
}

#[post("/play")]
fn play() -> String{
        SINK.with(|sink_cell| {
            let sink = sink_cell.borrow_mut();
            sink.set_volume(1.0);
            sink.play();
            thread::sleep(Duration::from_millis(100));
        });

        "success".to_string()
}

#[post("/load_songs", format="json", data="<my_track>")]
fn load_songs(my_track: Json<MyTrack>) {
    SINK.with(|sink_cell| {
        let track_list: Vec<Track> = my_track.0.track_list; 
        for track in track_list{
            let file = std::fs::File::open(&track.path.unwrap()).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            //let sink = sink_cell.borrow_mut();
            sink_cell.borrow_mut().append(source);
            println!("sink's length: {}", sink_cell.borrow_mut().len());
            sink_cell.borrow_mut().pause();
        }
    });
}


#[get("/get_songs")]
fn get_songs() -> Json<MyTrack> {
    let mut track_list: Vec<Track> = Vec::new();
    track_list = get_track_list();
    let tracks: MyTrack = MyTrack{track_list};
    Json(tracks)
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
        .mount("/", routes![pause, play, stop, index, load_songs, get_songs])
        .attach(Template::fairing())
}

fn main() {
    // Example playlist entry
/*
    let media_dir = Path::new("media/");
    let music_lib = get_map(&media_dir);

    match music_lib {
        Ok(a) => {
            println!("HashMap has {} values\n", a.len());
            for b in a.keys() {
                let track = a.get(b).unwrap();
                println!("{}", track.title);
            }
        }
        Err(_) => println!("ERROR READING MUSIC LIBRARY"),
    }*/

    rocket().launch();
}
