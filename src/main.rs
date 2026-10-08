use clap::Parser;
use rand::Rng;
use std::fs;
use std::io;

// The following Rust code was hand written in a small town in Antarctica

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Cli {
    #[arg(short = 'a', long = "add", value_name = "topic")]
    add: Option<String>,
}


fn main() {
    let args = Cli::parse();

    // as far as i am aware this is a simple way to do it.
    let list_path = shellexpand::tilde("~/.config/mooce/list").into_owned();
    let list_r = fs::read_to_string(list_path).expect("There is no file at ~/.config/mooce called \"list\" , please add a file there");

    match args.add {
        Some(topic) => {
            mooce::write_to_list(format!("{topic}"));
        },
        None => {
            let line_count = list_r.lines().count();
            if 1 > line_count {
                println!("Your list is empty! Add topic to it with mooce -a <topic>");
            } else {
                let rand_line = rand::thread_rng().gen_range(0..line_count);
                let line = list_r.lines().nth(rand_line);
                if let Some(e) = line {
                    // remove the some(" ") formatting from the text
                    println!("{e}");
                }
                println!("Complete this topic? (y/n)");
                let mut prompt = String::new();
                loop {
                    io::stdin().read_line(&mut prompt).expect("Failed to understand");
                    prompt = prompt.trim().to_lowercase();
                    if prompt.contains("y") {
                        mooce::remove_line(rand_line).expect("An error occured");
                        println!("Removed topic with id {rand_line}");
                        break
                    } else {
                        println!("You can find this again, the id of this is {:?}", rand_line);
                        break
                    }
                }
            }
        },
    }
}
