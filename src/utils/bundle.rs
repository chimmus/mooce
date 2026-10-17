use super::display::*;
use rand::Rng;

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
