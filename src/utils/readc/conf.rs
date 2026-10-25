use std::{fs};
use super::super::misc::*;

pub fn find_default() -> (String, bool) {
    let path = shellexpand::tilde("~/.config/mooce/mooce.conf").into_owned();
    if exist(&path) {
        let (list_r, success) = can_read(&path);
        if success {
            match read_default(&list_r) {
                Some(dpath) => {
                    let dpath = format!("~/.config/mooce/{}", dpath);
                    let dpath = shellexpand::tilde(&dpath).into_owned();
                    if exist(&dpath) {
                        return (dpath, true)
                    } else {
                        return (String::from("ignore"), false)
                    }
                },
                None => {
                    return (String::from("ignore"), false)
                }
            }
        }
    }
    return (String::from("ignore"), false)
}

fn can_read(path: &String) -> (String, bool) {
    match fs::read_to_string(&path) {
        Ok(list_r) => (list_r, true),
        Err(_) => (String::new(), false),
    }
}

fn read_default(list_r: &String) -> Option<String> {
    for line in list_r.lines() {
        let line_h = line.trim_start().to_lowercase();
        if line_h.starts_with("default=") {
            let def_path = &line.trim_start()[8..];
            let dpath: Vec<&str> = def_path.split_whitespace().collect();
            let dpath = dpath.join("\\ ");
            return Some(dpath)
        }
    }
    None
}
