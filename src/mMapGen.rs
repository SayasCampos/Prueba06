use std::collections::HashMap;
use std::path::Path;
use std::fs::read_dir;

mod track;
use track::Track;

pub fn get_map(dir: &String) -> Result<HashMap> {
    let mut hm = HashMap::new();
    let dir = Path::new(dir);
    
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let pstr = path.file_name().unwrap().to_str().unwrap().to_string();
            if !path.is_dir() {
                hm.insert(pstr, Track::new(path));
            }
            else {
                hm_opt = get_map(path.to_str()?);
                match hm_opt {
                    Some(hm_new) => hm.extend(hm_new),
                    None() => (),
                }
            }
        }
        hm
    }
    else{
        Err("Not a directory".to_string()))
    }
}
                
        


