extern crate rodio;

use std::io::BufReader;
use std::thread;
use std::time::Duration;

fn main() {
    let device = rodio::default_output_device().unwrap();

    let file = std::fs::File::open("examples/victory.mp3").unwrap();
    let victory = rodio::play_once(&device, BufReader::new(file)).unwrap();
    victory.set_volume(1.0);
    println!("victory");

    thread::sleep(Duration::from_millis(4500));
}
