use std::path::{Path, PathBuf};
//use id3::{Tag, Frame};
//use id3::frame::{Content, Picture, PictureType};
//use id3_image::extract_first_image;

#[derive(Serialize)]
pub struct Track {
    pub path: Option<PathBuf>,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub genre: String,
    pub year: i32,
    pub duration: u32,
    pub tags: Vec<String>,
    pub cover: Option<PathBuf>,
}

impl Track {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Track {
        let hard_code_file = Path::new("media/pauElliot.mp3");
        let temp_img = Path::new("static/img/temp.png");
        let temp_tag = id3::Tag::read_from_path(&hard_code_file).unwrap();
        let tag = id3::Tag::read_from_path(&file_path).unwrap_or(temp_tag);
        let pic = tag.pictures().next();
        if let Some(p) = pic {
            match image::load_from_memory(&p.data) {
                Ok(image) => {
                    image.save(&temp_img);
                }
                _ => println!("Couldn't load image"),
            };
        } else {
            println!("No art to load");
        }

        match id3::Tag::read_from_path(file_path.as_ref()) {
            Ok(file) => Track {
                path: Some(file_path.as_ref().to_owned()),
                title: file.title().unwrap_or("Unkown").to_string(),
                album: file.album().unwrap_or("Unknown").to_string(),
                artist: file.artist().unwrap_or("Unknown").to_string(),
                genre: file.genre().unwrap_or("Unknown").to_string(),
                year: file.year().unwrap_or(0),
                duration: file.duration().unwrap_or(0),
                tags: Vec::new(),
                cover: Some(Path::new("media/test.png").to_path_buf()),
            },
            Err(_) => Track {
                path: None,
                title: "unknown".to_string(),
                album: "unknown".to_string(),
                artist: "unknown".to_string(),
                genre: "unknown".to_string(),
                year: 0,
                duration: 0,
                tags: Vec::new(),
                cover: None,
            },
        }
    }
}
