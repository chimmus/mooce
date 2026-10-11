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
    /// Which list file to target?
    #[arg(short = 't', long = "target", value_name = "file name")]
    target: Option<String>,
}


fn main() {
    let args = Cli::parse();

    let (list_r, path) = match args.target {
        Some(ref file) => {
            let path = format!("~/.config/mooce/{}", file);
            let path_check = shellexpand::tilde(&path).into_owned();
            let list_r = fs::read_to_string(&path_check)
                .expect("The file you requested was not found, please add a file there");
            (list_r, path_check)
        },
        None => {
            let list_path = shellexpand::tilde("~/.config/mooce/list").into_owned();
            let list_r = fs::read_to_string(&list_path)
                .expect("There is no file at ~/.config/mooce called \"list\" , please add a file there");
            (list_r, list_path)
        },
    };

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
            let _ = mooce::write_to_list(format!("{topic}"), path);
        },
        None => {
            if !args.list && !args.del.is_some() && !args.read.is_some() && !args.target.is_some() {
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
            mooce::print(ln, list_r);
        },
        _ => {},
    }

    println!("────────────────────────");

}
