use std::fs;
use super::misc::exist;

pub fn make(file_name: &Option<String>) {
    match file_name {
        Some(file) => {
            if file.contains(".") {
                println!("Unfortunately mooce wont allow this, we do not permit . inside of the list name");
            } else {
                let check = format!("~/.config/mooce/{}", file);
                let path_check = shellexpand::tilde(&check).into_owned();
                if exist(&path_check) {
                    println!("There seems to be a file there with the same name");
                } else {
                    let parent = shellexpand::tilde("~/.config/mooce/").into_owned();
                    fs::create_dir_all(&parent).expect("Could not create ~/.config/mooce");

                    match fs::File::create(&path_check) {
                        Ok(_) => println!("File successfully created!"),
                        Err(errpr) => println!("error: {errpr}"),
                    }
                }
            }
        },
        None => {},
    }
}
