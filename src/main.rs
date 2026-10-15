mod utils;

use clap::Parser;
use rand::Rng;
use utils::{display::{show, ls}, update::{write_list}, delete::*, create::*, misc::*};

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
    /// List all lists
    #[arg(short = 'L', long = "list-file")]
    listf: bool,
    /// Delete entry at the index
    #[arg(short = 'd', long = "delete", value_name = "index")]
    del: Option<usize>,
    /// Delete entire list (no confirmation)
    #[arg(short = 'D', long = "delete-list", value_name = "file name")]
    dell: Option<String>,
    /// Read entry at index
    #[arg(short = 'r', long = "read", value_name = "index")]
    read: Option<usize>,
    /// Which list file to target?
    #[arg(short = 't', long = "target", value_name = "file name")]
    target: Option<String>,
    /// Create a list
    #[arg(short = 'c', long = "create", value_name = "file name")]
    create: Option<String>,
}


fn main() {
    let args = Cli::parse();

    rmr(&args.dell);
    make(&args.create);
    let (list_r, path) = target(&args.target);

    if args.list {
        // println!("{}", list_r);
        let mut i: usize = 0;
        println!("────────────────────────");
        for line in list_r.lines() {
            println!("[{i}] {line}");
            i += 1;
        }
    }

    if args.listf {
        println!("────────────────────────");
        ls();
    }

    match args.add {
        Some(topic) => {
            println!("────────────────────────");
            let _ = write_list(format!("{topic}"), &path);
        },
        None => {
            if !args.list && !args.del.is_some() && !args.read.is_some() && !args.create.is_some() && !args.dell.is_some() && !args.listf {
                let line_count = list_r.lines().count();
                // really inefficient ^
                if 1 > line_count {
                    println!("────────────────────────");
                    println!("Your list is empty! Add topic to it with mooce -a <topic>");
                } else {
                    println!("────────────────────────");
                    show(rand::thread_rng().gen_range(0..line_count), &path);
                }
            }
        },
    }

    match args.del {
        Some(ln) => {
            println!("────────────────────────");
            delete_ln(ln, &path);
        },
        _ => {},
    }

    match args.read {
        Some(ln) => {
            println!("────────────────────────");
            show(ln, &path);
        },
        _ => {},
    }

    println!("────────────────────────");

}
