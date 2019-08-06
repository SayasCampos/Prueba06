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

use qr2term::print_qr; // for later use when IP is no longer static

//////////////////basis for the wrapped code found here
//////////////////https://stackoverflow.com/questions/19605132/is-it-possible-to-use-global-variables-in-rust
thread_local!(static DEVICE: RefCell<rodio::Device> = RefCell::new(rodio::default_output_device().unwrap()));
thread_local!(static SINK: RefCell<rodio::Sink> = RefCell::new(rodio::Sink::new(&rodio::default_output_device().unwrap())));
/////////////////end wrapped code

/////////////////////////////////////////////
////MyTrack:
////    This struct is used in order to pass 
////    a vector of tracks back and forth
////    between the front end and the back end
////    as a JSON object. 
//// Function Author: Paul Hubbard
/////////////////////////////////////////////
#[derive(Serialize, Deserialize)]
struct MyTrack {
    track_list: Vec<Track>,
}

fn change_cover<P: AsRef<Path>>(file_path: P) {
    //fn change_cover new<P: AsRef<Path>> (file_path: P) {
    let temp_img = Path::new("static/img/temp.png");
    let tag = id3::Tag::read_from_path(&file_path).unwrap();
    let pic = tag.pictures().next();
    if let Some(p) = pic {
        match image::load_from_memory(&p.data) {
            Ok(image) => {
                image.save(&temp_img).unwrap();
            }
            _ => println!("Couldn't load image"),
        };
    } else {
        println!("No art to load");
    }
}
///////////////////////////////////////////////////////////////////
////get_track_list:
////   This returns a list of all available tracks in a random 
////   order.
////  Function Author: Max Smiley / Paul Hubbard
///////////////////////////////////////////////////////////////////
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
///////////////////////////////////////////////////////////////////
////get_track:
//// This gets a single track from the list of available tracks
//// based on whatever track name gets passed in. 
////    Parameters: 
////        track_name: This is the name of the track
////                    you wish to get from the available
////                    tracks. 
////    Function Author: Max Smiley / Paul Hubbard
//////////////////////////////////////////////////////////////////
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
/////////////////////////////////////////////////
////pause:
////    This pauses the current audio being played
////    in the audio sink. 
////       Function Author: Paul Hubbard
/////////////////////////////////////////////////
#[post("/pause")]
fn pause() {
    SINK.with(|sink_cell| {
        let sink = sink_cell.borrow_mut();
        sink.pause();
    });
}
////////////////////////////////////////////////
////stop:
//// This gets called by a post from the front end
//// and it removes all of mp3's currently in the
//// sink, while also stopping the audio. 
////       Function Author: Paul Hubbard    
///////////////////////////////////////////////
#[post("/stop")]
fn stop() {
    SINK.with(|sink_cell| {
        sink_cell.borrow_mut().stop();
        let new_sink: RefCell<rodio::Sink> = RefCell::new(rodio::Sink::new(&rodio::default_output_device().unwrap()));
        sink_cell.swap(&new_sink);
    });
}
////////////////////////////////////////////////
////play:
//// This function is called from a post coming 
//// from the front end, and it starts the 
//// audio sink list, then returns a string
//// telling the front end it was a success.
////        Function Author: Paul Hubbard
////////////////////////////////////////////////
#[post("/play")]
fn play() -> String {
    SINK.with(|sink_cell| {
        let sink = sink_cell.borrow_mut();
        sink.set_volume(1.0);
        sink.play();
    });

    "success".to_string()
}


////////////////////////////////////////////////
////radio:
//// This function plays a internet radio
//// station when given a correct  web address
////    Parameters:
////        url: webaddress of internet radio
////                  station.
////
//// Function Author:
////    Christopher Teters
///////////////////////////////////////////////
#[post("/radio", data = "<url>")]
fn radio(url: String) -> String{
    gst::init();
    let mut playbin = gst::PlayBin::new("audio_player").expect("Couldn't create playbin");
    playbin.set_uri(&url);
    playbin.play();
    "success".to_string()
}


////////////////////////////////////////////////
////load_songs:
//// This function receives a playlist
//// as a JSON object from the frontend
//// and loads the songs from their mp3
//// source into a sink that will play
//// the audio. 
////    Parameters:
////        my_track: This is the JSON object
////                  that gets sent in from the 
////                  user. 
////    Function Author:
////        Paul Hubbard
/////////////////////////////////////////////////
#[post("/load_songs", format = "json", data = "<my_track>")]
fn load_songs(my_track: Json<MyTrack>) {
    SINK.with(|sink_cell| {
        let track_list: Vec<Track> = my_track.0.track_list;
        for track in track_list {
            let file = std::fs::File::open(&track.path.unwrap()).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            let new_source = source.buffered();
            //let sink = sink_cell.borrow_mut();
            sink_cell.borrow_mut().append(new_source.clone());
            sink_cell.borrow_mut().pause();
            println!(
                "sink's length: {}\nsong title: {}",
                sink_cell.borrow_mut().len(),
                track.title
            );
        }
    });
}

//////////////////////////////////////////////////
////get_songs:
//// This function returns all of the
//// current mp3's available to play
//// and sends them as a JSON object
//// back to the front end.
////    Function Author: Paul Hubbard
//////////////////////////////////////////////////
#[get("/get_songs")]
fn get_songs() -> Json<MyTrack> {
    let mut track_list: Vec<Track> = Vec::new();
    track_list = get_track_list();
    let tracks: MyTrack = MyTrack { track_list };
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
        .mount(
            "/",
            routes![pause, play, stop, index, load_songs, get_songs, radio],
        )
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

        print_qr("http://192.168.1.32:8888");
    rocket().launch();
}
