use std::fs;
use super::readc::conf::*;
use super::create::*;
use std::path::Path;

pub fn exist(path: &String) -> bool {
    match fs::exists(path) {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => false,
    }
}

pub fn target(target: &Option<String>, mpath: &String) -> (String, String) {
    match target {
        Some(file) => {
            let path = format!("{}{}", mpath, file);
            let path_check = shellexpand::tilde(&path).into_owned();
            if exist(&path_check) {
                let list_r = fs::read_to_string(&path_check)
                    .expect("The file you requested was not found, please add a file there");
                println!("────────────────────────");
                println!("Targetting {file}");
                (list_r, path_check)
            } else {
                println!("The file you requested didn't exist!");
                do_default()
            }
        },
        None => {
            do_default()
        },
    }
}

pub fn check_dir (path: &String) -> bool {
    Path::new(path).is_dir()
}

fn do_default() -> (String, String) {
    let (list_path, success) = find_default();
    if success {
        let list_r = fs::read_to_string(&list_path)
            .expect("There is no file at ~/.config/mooce , please add a file there");
        (list_r, list_path)
    } else {
        let list_path = shellexpand::tilde("~/.config/mooce/list").into_owned();
        if exist(&list_path) {
            let list_r = fs::read_to_string(&list_path)
                .expect("There is no file at ~/.config/mooce called list , please add a file there");
            (list_r, list_path)
        } else {
            make(&Some(String::from("list")));
            let list_path = shellexpand::tilde("~/.config/mooce/list").into_owned();
            let list_r = fs::read_to_string(&list_path)
                .expect("There is no file at ~/.config/mooce called list , please add a file there");
            (list_r, list_path)
        }
    }
}
