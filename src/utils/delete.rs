use std;
use super::misc::exist;

pub fn remove(line_i: usize, path: &String) -> std::io::Result<()> {
    match  std::fs::read_to_string(&path) {
        Ok(_list_r) => {
            let list_w = std::fs::read_to_string(&path)?;
            let mut lines: Vec<String> = list_w.lines().map(String::from).collect();

            if line_i < lines.len() {
                lines.remove(line_i);
            }

            let mut out = lines.join("\n");
            if !out.is_empty() {
                out.push('\n');
            }
            std::fs::write(&path, out)?;
        },
        Err(error) => {
            println!("An error occured {error}");
        },
    }
    Ok(())
}

pub fn rmr(target: &Option<String>) {
    match target {
        Some(file) => {
            let check = format!("~/.config/mooce/{}", file);
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
                    println!("Error occured")
                }
            }
        },
        None => {},
    }
}
