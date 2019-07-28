use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

pub mod track;
use track::Track;

pub fn get_map(dir: &Path) -> io::Result<HashMap<String, Track>> {
    let mut hm = HashMap::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let matchpath = Path::new(&path);

            if !path.is_dir() {
                let fname = match matchpath.file_name() {
                    None => None,
                    Some(v) => v.to_str(),
                };
                match fname {
                    None => None,
                    Some(s) => {
                        if s.ends_with(".mp3") {
                            let p = Path::new(&path);
                            hm.insert(s.to_owned(), Track::new(&p))
                        } else {
                            None
                        }
                    }
                };
            } else {
                let hm_opt = get_map(&path);
                match hm_opt {
                    Ok(hm_new) => hm.extend(hm_new),
                    Err(_) => (),
                }
            }
        }
        Ok(hm)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "not a directory!"))
    }
}
