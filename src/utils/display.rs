use std::{io, fs};
use super::delete::remove;

pub fn show(ln: usize, path: &String) {
    let list_r = fs::read_to_string(&path).expect("File requested was not found");
    if ln > list_r.lines().count() {
        println!("{ln} does not exist!");
    } else {
        let line = list_r.lines().nth(ln);
        if let Some(e) = line {
            println!("[{ln}] {e}")
        }
        rprompt(ln, &path);
    }
}


fn rprompt(ln: usize, path: &String) {
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
