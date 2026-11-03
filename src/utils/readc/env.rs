use std::env;
use super::super::misc::*;

pub fn detect() -> (String, bool) {
    match env::var("MPATH") {
        Ok(path) => {
            let path_dir = shellexpand::tilde(&path).into_owned();
            if check_dir(&path_dir) && exist(&path_dir) {
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
    let list_path = shellexpand::tilde("~/.config/mooce/list").into_owned();
    list_path
}
