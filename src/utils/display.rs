use std::{io, fs};
use super::delete::remove;
use super::misc::exist;

pub fn show(ln: usize, path: &String) -> (String, bool) {
    if exist(&path){
        match fs::read_to_string(&path) {
            Ok(list_r) => {
                if ln > list_r.lines().count() {
                    (format!("{ln} does not exist!"), false)
                } else {
                    let line = list_r.lines().nth(ln);
                    if let Some(e) = line {
                        (format!("[{}] {}", ln, e), true)
                    } else {
                        (format!("unknown error occured"), true)
                    }
                }
            },
            Err(error) => {
                (format!("an error occured {error}"), false)
            },
        }
    } else {
        (format!("list doesnt exist!"), false)
    }
}


pub fn rprompt(ln: usize, path: &String) {
    println!("Complete this topic? (y/n)");
    let mut prompt = String::new();
    loop {
        io::stdin().read_line(&mut prompt).expect("Failed to understand");
        prompt = prompt.trim().to_lowercase();
        if prompt.contains("y") {
            remove(ln, &path).expect("An error occured");
            println!("Removed topic with id {ln}");
            break
        } else {
            println!("You can find this again, the id of this is {:?}", ln);
            break
        }
    }
}

pub fn ls() {
    let path_c = shellexpand::tilde("~/.config/mooce/").into_owned();
    for path in fs::read_dir(&path_c).expect("an error occured") {
        let path_e = path.expect("Could not read entry");
        let path_c = path_e.file_name().to_string_lossy().into_owned();
        if !path_c.contains(".") {
            println!("{}", path_c);
        }
    }
}

