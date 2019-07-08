extern crate rodio;     // digital audio coding
extern crate iron;      // high level web framwork
extern crate router;    // routing for the iron crate
extern crate urlencoded;use std::str::FromStr; // middleware for iron
#[macro_use] extern crate mime; // basic web graphics

use std::io::BufReader;
use std::thread;
use std::time::Duration;
use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_from, "index");
    router.post("/player", player_funct, "player");

    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();

}

fn get_from(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>Rustadio Media Player</title>
        <form action="/player" method="post">
            <button type="submit">Prev</button>
            <button type="submit">Play/Pause</button>
            <button type="submit">Stop</button>
            <button type="submit">Next</button>
            <button type="submit">Vol +</button>
            <button type="submit">Vol -</button>
        </form>
    "#);

    Ok(response)
}

fn player_funct(request: &mut Request) -> IronResult<Response> {


    let device = rodio::default_output_device().unwrap();

    let file = std::fs::File::open("examples/victory.mp3").unwrap();
    let victory = rodio::play_once(&device, BufReader::new(file)).unwrap();
    victory.set_volume(1.0);
    println!("victory");

    thread::sleep(Duration::from_millis(4500));

    
    let mut response = Response::new();


    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("Playing track\n"));
    Ok(response)
}
