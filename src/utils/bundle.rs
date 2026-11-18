use super::display::*;
use rand::Rng;
use super::delete::*;

/* A bundle of the display::show function combined with RNG, 
this is used in the uh default utility
of the program */
pub fn default_rand(line_count: usize, path: &String, yes: bool, no: bool) {
    if 1 > line_count {
        println!("────────────────────────");
        println!("Your list is empty! Add topic to it with mooce -a <topic>");
    } else {
        println!("────────────────────────");
        let rand_line = rand::thread_rng().gen_range(0..line_count);
        let (output, success) = show(rand_line, &path);
        if success {
            println!("{}", output);
            rprompt(rand_line, &path, yes, no);

        } else {
            eprintln!("ERROR! {output}");
        }
    }
}

pub fn delete_ln(line_number: usize, path: &String) {
    // This delete is more expressive
    match std::fs::read_to_string(&path) {
        Ok(list_r) => {
            let line_count = list_r.lines().count();
            if line_number > (line_count - 1) {
                eprintln!("{line_number} does not exist!");
            } else {
                if iremove(line_number, &path) {
                    println!("[{line_number}] has been removed"); 
                } else {
                    eprintln!("error occured while removing [{line_number}]")
                }
            }
        },
        Err(error) => {
            eprintln!("Unexpected error occured {error}");
        },
    }
}
