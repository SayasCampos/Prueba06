# rustadio
The current end state goal of this project is for it to be
a simple media player (or radio simulator) that can be ran in the background
while a computer is in use by another application (like
a video game) at full screen. The webb app in turn could
be controlled and viewed by another media device, such as
a tablet or smart phone, giving the user a higher level
of immersion and control with an external device that is
functional to the computer.

This program is still in development and can currently be used as a simple webapp
based media player to play common digital audio coding formats such as mp3 and
[HTTP Live Streaming](https://en.wikipedia.org/wiki/HTTP_Live_Streaming)
 of different radio stations using the Rust programming language.

The software itself won't likely see any further development until the backend
[Rocket](https://rocket.rs/) includes asynchronous support.
Later goals however might include the implementation
and use file formats such as [M3U](https://en.wikipedia.org/wiki/M3U), for providing use of
playlists. Long term project ideas currently consider the
implementation and use of media hotkeys such as
Play/Pause, Stop, Next and previous.

## Build and Run
This program utilizes gstreamer libraries that must 
be installed on linux in order for it to run. To install, 
please run the following...

    sudo apt-get install libgstreamer-plugins-base1.0-dev
    sudo apt-get install libsdl2-dev
    sudo apt-get install libasound2-dev
    sudo apt-get install libgstreamer-plugins-bad1.0-dev
    sudo apt-get install libgstreamer1.0-0 gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-doc gstreamer1.0-tools gstreamer1.0-x gstreamer1.0-alsa gstreamer1.0-gl gstreamer1.0-gtk3 gstreamer1.0-qt5 gstreamer1.0-pulseaudio


This program uses Rocket, which requires Rust nightly. To
build rustadio, first enable nightly if not already enabled
and build the library with `cargo build`. You can
run the program with `cargo run`.

    rustup default nightly
    cargo build
    cargo run

To build or run an optimized version, use `cargo --release`.

Run `cargo test` to do some simple testing.

Run `cargo run` from the main rustadio folder to run the 
program. The program will not function properly if you 
run it from any of its sub-directories. 

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.

## Contact
[Christopher Teters](https://github.com/cteters),
cteters@pdx.edu

[Max Smiley](https://github.com/maxjaspersmiley),
smiley6@pdx.edu

[Paul Hubbard](https://github.com/phubbard67),
phubbard@pdx.edu
