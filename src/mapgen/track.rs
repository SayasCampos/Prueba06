use std::path::Path;
//use std::path::{Path, PathBuf};
//use id3::frame::{Picture, PictureType};
//use id3_image::extract_first_image;

pub struct Track {
    //pub path:   PathBuf,
    
    pub path: String,

    pub title: String,
    pub album: String,
    pub artist: String,
    pub genre: String,
    pub year: i32,
    pub duration: u32,
    pub tags: Vec<String>,
    //albumArt
}

impl Track {
//    pub fn new<P: AsRef<Path>>(file_path: P) -> Track {
    pub fn new(file_path: &Path) -> Track {

//        let file = id3::Tag::read_from_path(file_path).unwrap().clone();

        match id3::Tag::read_from_path(file_path) {
            Ok(file)   => {
                Track {
                    path:       file_path.to_str().unwrap().to_string(),
                    title:      file.title().unwrap_or("Unkown").to_string(),
                    album:      file.album().unwrap_or("Unknown").to_string(),
                    artist:     file.artist().unwrap_or("Unknown").to_string(),
                    genre:      file.genre().unwrap_or("Unknown").to_string(),
                    year:       file.year().unwrap_or(0),
                    duration:   file.duration().unwrap_or(0),
                    tags:       Vec::new(),
                    //albumArt
                }
            },
            Err(_)     => {
                Track {
                    path:       file_path.to_str().unwrap().to_string(),
                    title:      "unknown".to_string(),
                    album:      "unknown".to_string(),
                    artist:     "unknown".to_string(),
                    genre:     "unknown".to_string(), 
                    year:       0,
                    duration:   0,
                    tags:       Vec::new(),
                }
            },
        }
    }
}
