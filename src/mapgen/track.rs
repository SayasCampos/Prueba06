use mp3_duration;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    pub path: Option<PathBuf>,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub genre: String,
    pub year: i32,
    pub duration: std::time::Duration,
    pub tags: Vec<String>,
    pub cover: Option<PathBuf>,
}

impl Default for Track {
    fn default() -> Self {
        Track {
            path: None,
            title: "unknown".to_string(),
            album: "unknown".to_string(),
            artist: "unknown".to_string(),
            genre: "unknown".to_string(),
            year: 0,
            duration: Duration::new(0, 0),
            tags: Vec::new(),
            cover: None,
        }
    }
}

impl Track {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Track {
        match id3::Tag::read_from_path(file_path.as_ref()) {
            Ok(file) => Track {
                path: Some(file_path.as_ref().to_owned()),
                title: file.title().unwrap_or("Unkown").to_string(),
                album: file.album().unwrap_or("Unknown").to_string(),
                artist: file.artist().unwrap_or("Unknown").to_string(),
                genre: file.genre().unwrap_or("Unknown").to_string(),
                year: file.year().unwrap_or(0),
                duration: mp3_duration::from_path(file_path.as_ref()).unwrap(),
                tags: Vec::new(),
                cover: Some(Path::new("static/img/album/current-cover.png").to_path_buf()),
            },
            Err(_) => Default::default(),
        }
    }
}
