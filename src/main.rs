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

use qr2term::print_qr;

//////////////////basis for the wrapped code found here
//////////////////https://stackoverflow.com/questions/19605132/is-it-possible-to-use-global-variables-in-rust

thread_local!(static DEVICE: RefCell<rodio::Device> = RefCell::new(rodio::default_output_device().unwrap()));
thread_local!(static SINK: RefCell<rodio::Sink> = RefCell::new(rodio::Sink::new(&rodio::default_output_device().unwrap())));
/////////////////end wrapped code

#[derive(Serialize)]
struct MyTrack {
    track_list: Vec<Track> 
}

#[post("/stop")]
fn stop(){
        SINK.with(|sink_cell| {
            let sink = sink_cell.borrow_mut();
            sink.play();
            thread::sleep(Duration::from_millis(340000));
        });
}

#[post("/media/<id>")]
fn play_victory(id: &rocket::http::RawStr) -> String {
    //let device = rodio::default_output_device().unwrap();
    let mut path = "media/".to_string();
    path.push_str(id.as_str());
    println!("{}", path);
    let _current_song_path = Path::new(&path);
    let duration = mp3_duration::from_path(&path).unwrap();
    //let mill_dur = duration.as_millis() as u64;
    let _current_song: Track = Track::new(_current_song_path);
    println!("{:?}", duration);
    let file = std::fs::File::open(&path).unwrap();

//////////////////basis for the wrapped code found here, and any code) resembling this code
//////////////////(any code calling SINK.with()) can be found here
//////////////////https://stackoverflow.com/questions/19605132/is-it-possible-to-use-global-variables-in-rust
    //DEVICE.with(|device_cell| {
        SINK.with(|sink_cell| {
            let sink = sink_cell.borrow_mut();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            //TODO: Pause it here. This will no longer be the 
            //play function, but the load songs function, or we 
            //can move a load songs functionality to another function.
            sink.append(source);
        });
    //});
////////////////end wrapped code

    println!("{}", _current_song.title);

    _current_song.title.to_string()
}

#[get("/get_songs")]
fn get_songs() -> Json<MyTrack> {
    let _current_song_path = Path::new("media/victory.mp3");
    let _current_song: Track = Track::new(_current_song_path);
    let _next_song_path = Path::new("media/pauElliot.mp3");
    let _next_song: Track = Track::new(_next_song_path);

    let mut track_list: Vec<Track> = Vec::new();
    track_list.push(_current_song);
    track_list.push(_next_song);
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
        .mount("/", routes![stop, index, play_victory, get_songs])
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
        }
        Err(_) => println!("ERROR READING MUSIC LIBRARY"),
    }

//    print_qr("http://192.168.1.32:8000");
    rocket().launch();
}
