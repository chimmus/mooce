use super::display::*;
use rand::Rng;
use super::delete::*;

/* A bundle of the display::show function combined with RNG, 
this is used in the uh default utility
of the program */
pub fn default_rand(line_count: usize, path: &String) {
    if 1 > line_count {
        println!("────────────────────────");
        println!("Your list is empty! Add topic to it with mooce -a <topic>");
    } else {
        println!("────────────────────────");
        let rand_line = rand::thread_rng().gen_range(0..line_count);
        let (output, success) = show(rand_line, &path);
        if success {
            println!("{}", output);
            rprompt(rand_line, &path);

        } else {
            println!("ERROR! {output}");
        }
    }
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
