use std::env;
use super::super::misc::*;

pub fn detect() -> (String, bool) {
    match env::var("MPATH") {
        Ok(path) => {
            let mut path_dir = shellexpand::tilde(&path).into_owned();
            if check_dir(&path_dir) && exist(&path_dir) {
                if !path_dir.ends_with('/') {
                    path_dir.push_str("/");
                }
                (path_dir, false)
            } else { 
                (default(), true)
            }
        },
        Err(_) => {
            (default(), true) 
        }
    }
}

fn default() -> String {
    let list_path = shellexpand::tilde("~/.config/mooce/").into_owned();
    list_path
}
