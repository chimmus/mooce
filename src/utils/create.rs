use std::fs;
use super::misc::exist;

pub fn make(file_name: &Option<String>, mpath: &String) {
    match file_name {
        Some(file) => {
            if file.contains(".") {
                eprintln!("Unfortunately mooce wont allow this, we do not permit . inside of the list name");
            } else {
                let check = format!("{}{}", mpath, file);
                let path_check = shellexpand::tilde(&check).into_owned();
                if exist(&path_check) {
                    eprintln!("There seems to be a file there with the same name");
                } else {
                    let parent = shellexpand::tilde(&mpath).into_owned();
                    match fs::create_dir_all(&parent) {
                        Ok(_) => (),
                        Err(error) => eprintln!("error {error}")
                    }

                    match fs::File::create(&path_check) {
                        Ok(_) => println!("File successfully created!"),
                        Err(errpr) => eprintln!("error: {errpr}"),
                    }
                }
            }
        },
        None => {},
    }
}
