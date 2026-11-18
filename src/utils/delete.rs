use std;
use super::misc::exist;

pub fn iremove(line_i: usize, path: &String) -> bool {
    match std::fs::read_to_string(&path) {
        Ok(_list_r) => {
            let list_w = match std::fs::read_to_string(&path) {
                Ok(write) => write,
                Err(e) => {
                    eprintln!("error occured: {e}");
                    return false;
                },
            };
            let mut lines: Vec<String> = list_w.lines().map(String::from).collect();

            if line_i < lines.len() {
                lines.remove(line_i);
            }

            let mut out = lines.join("\n");
            if !out.is_empty() {
                out.push('\n');
            }
            std::fs::write(&path, out).is_ok()
        },
        Err(error) => {
            eprintln!("An error occured {error}");
            false
        },
    }
}

// remove file
pub fn rmr(target: &Option<String>, mpath: &String) {
    match target {
        Some(file) => {
            let check = format!("{}{}", mpath, file);
            let path_check = shellexpand::tilde(&check).into_owned();
            if !exist(&path_check) {
                println!("The file didn't exist so we didn't need to do anything");
            } else {
                let success = match std::fs::remove_file(&path_check) {
                    Ok(()) => true,
                    Err(_) => false,
                };
                if success {
                    println!("File successfully deleted!");
                } else {
                    eprintln!("Error occured")
                }
            }
        },
        None => {},
    }
}
