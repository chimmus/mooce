use clap::Parser;
use rand::Rng;
use std::fs;

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
            todo!("write to the list file");
        },
        None => {
            let line_count = list_r.lines().count();
            let line = list_r.lines().nth(rand::thread_rng().gen_range(0..line_count));
            if let Some(e) = line {
                // remove the some(" ") formatting from the text
                println!("{e}");
            }
        },
    }
}
