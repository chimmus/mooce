use std::fs;

pub fn exist(path: &String) -> bool {
    match fs::exists(path) {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => false,
    }
}

pub fn target(target: &Option<String>) -> (String, String) {
    match target {
        Some(file) => {
            let path = format!("~/.config/mooce/{}", file);
            let path_check = shellexpand::tilde(&path).into_owned();
            if exist(&path_check) {
                let list_r = fs::read_to_string(&path_check)
                    .expect("The file you requested was not found, please add a file there");
                println!("────────────────────────");
                println!("Targetting {file}");
                (list_r, path_check)
            } else {
                let path_check = shellexpand::tilde("~/.config/mooce/list").into_owned();
                println!("Unfortunately the file you requested doesn't exist");
                let list_r = fs::read_to_string(&path_check)
                    .expect("There is no file at ~/.config/mooce called \"list\" , please add a file there");

                (list_r, path_check)
            }
        },
        None => {
            let list_path = shellexpand::tilde("~/.config/mooce/list").into_owned();
            let list_r = fs::read_to_string(&list_path)
                .expect("There is no file at ~/.config/mooce called \"list\" , please add a file there");
            (list_r, list_path)
        },
    }
}
