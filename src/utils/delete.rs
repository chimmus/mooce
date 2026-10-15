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

pub fn delete_ln(line_number: usize, path: &String) {
    // This delete is more expressive
    match std::fs::read_to_string(&path) {
        Ok(list_r) => {
            let line_count = list_r.lines().count();
            if line_number > (line_count - 1) {
                println!("{line_number} does not exist!");
            } else {
                remove(line_number, &path).expect("Error occured while removing line");
                println!("[{line_number}] has been removed"); // Not sure about how .expect works, 
            }
        },
        Err(error) => {
            println!("Unexpected error occured {error}");
        },
    }
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
