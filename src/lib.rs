use utils::delete::remove;

use std::fs;

pub mod utils;

pub fn delete_ln(line_number: usize, list_path: &String) {
    // This delete is more expressive
    let list_r = fs::read_to_string(&list_path).expect("File requested was not found");
    let line_count = list_r.lines().count();
    if line_number > (line_count - 1) {
        println!("{line_number} does not exist!");
    } else {
        remove(line_number, &list_path).expect("Error occured while removing line");
        println!("[{line_number}] has been removed"); // Not sure about how .expect works, 
    }
}

pub fn exist(path: &String) -> bool {
    match fs::exists(path) {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => false,
    }
}

