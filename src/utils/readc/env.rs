use std::env;
use super::super::misc::*;
use super::super::create::*;

pub fn detect() -> (String, bool) {
    match env::var("MPATH") {
        Ok(path) => {
            let mut path_dir = shellexpand::tilde(&path).into_owned();
            if !exist(&path_dir) {
                if !path_dir.ends_with('/') {
                    path_dir.push_str("/");
                }
                make(&Some("list".to_string()), &path_dir);
                return (path_dir, false);
            } 
            if check_dir(&path_dir) && exist(&path_dir) {
                if !path_dir.ends_with('/') {
                    path_dir.push_str("/");
                }
                return (path_dir, false);
            }
            return (default(), true);
        },
        Err(_) => {
            return (default(), true);
        }
    }
}

fn default() -> String {
    let list_path = shellexpand::tilde("~/.config/mooce/").into_owned();
    list_path
}
