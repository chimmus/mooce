use clap::Parser;
use std::fs;

// The following Rust code was hand written in a small town in Antarctica

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Cli {
    /// Add topic
    #[arg(short = 'a', long = "add", value_name = "topic")]
    add: Option<String>,
    /// List
    #[arg(short = 'l', long = "list")]
    list: bool,
    /// Delete entry at the index
    #[arg(short = 'd', long = "delete", value_name = "index")]
    del: Option<usize>,
    /// Read entry at index
    #[arg(short = 'r', long = "read", value_name = "index")]
    read: Option<usize>,
}


fn main() {
    let args = Cli::parse();

    // as far as i am aware this is a simple way to do it.
    let list_path = shellexpand::tilde("~/.config/mooce/list").into_owned();
    let list_r = fs::read_to_string(list_path)
        .expect("There is no file at ~/.config/mooce called \"list\" , please add a file there");

    if args.list {
        // println!("{}", list_r);
        let mut i: usize = 0;
        println!("────────────────────────");
        for line in list_r.lines() {
            println!("[{i}] {line}");
            i += 1;
        }
    }

    match args.add {
        Some(topic) => {
            println!("────────────────────────");
            let _ = mooce::write_to_list(format!("{topic}"));
        },
        None => {
            if !args.list && !args.del.is_some() && !args.read.is_some() {
                let line_count = list_r.lines().count();
                // really inefficient ^
                if 1 > line_count {
                    println!("────────────────────────");
                    println!("Your list is empty! Add topic to it with mooce -a <topic>");
                } else {
                    println!("────────────────────────");
                    mooce::print_r(line_count);
                }
            }
        },
    }

    match args.del {
        Some(ln) => {
            println!("────────────────────────");
            mooce::delete_ln(ln);
        },
        _ => {},
    }

    match args.read {
        Some(ln) => {
            println!("────────────────────────");
            mooce::print(ln);
        },
        _ => {},
    }

    println!("────────────────────────");

}
