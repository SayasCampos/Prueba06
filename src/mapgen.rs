use std::io;
use std::collections::HashMap;
use std::path::Path;
use std::fs;

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
                    None    => None,
                    Some(v) => v.to_str(),
                };
                match fname {
                    None    => None,
                    Some(s) => {
                        let p = Path::new(&path);
                        hm.insert(s.to_owned(), Track::new(&p))
                    },
                };
            }

            /*
             * this else clause deals with directory support.
             * it's not currently functioning and I consider it
             * lower priority. On the list, though.

            else {
                hm_opt = get_map(path.to_str()?);
                match hm_opt {
                    Some(hm_new) => hm.extend(hm_new),
                    None() => (),
                }
            }
            */
        }
        Ok(hm)
    }
    else{
        Err(io::Error::new(io::ErrorKind::Other, "not a directory!"))
    }
}
                
        


