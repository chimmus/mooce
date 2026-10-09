use clap::Parser;
use std::fs;

// The following Rust code was hand written in a small town in Antarctica

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Cli {
    #[arg(short = 'a', long = "add", value_name = "topic")]
    add: Option<String>,
    #[arg(short = 'l', long = "list")]
    list: bool,
}


fn main() {
    let args = Cli::parse();

    // as far as i am aware this is a simple way to do it.
    let list_path = shellexpand::tilde("~/.config/mooce/list").into_owned();
    let list_r = fs::read_to_string(list_path).expect("There is no file at ~/.config/mooce called \"list\" , please add a file there");

    if args.list {
        // println!("{}", list_r);
        let mut i: usize = 0;
        for line in list_r.lines() {
            println!("[{i}] {line}");
            i += 1;
        }
    } else {
        match args.add {
            Some(topic) => {
                let _ = mooce::write_to_list(format!("{topic}"));
            },
            None => {
                let line_count = list_r.lines().count();
                if 1 > line_count {
                    println!("Your list is empty! Add topic to it with mooce -a <topic>");
                } else {
                    mooce::print_r(line_count);
                }
            },
        }
    }

}
