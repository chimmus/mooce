mod utils;

use clap::Parser;
use utils::{display::{show, ls, rprompt}, update::{write_list}, delete::*, create::*, misc::*, bundle::*};

// The following Rust code was hand written in a small town in Antarctica

#[derive(Parser, Debug)]
#[command(version, about, long_about = "
mooce is quite unintuitive to use. There is no set way to use it.\nThe program comes with a few basic utilities and is probably quite slow")]

struct Cli {
    /// Add topic
    #[arg(short = 'a', long = "add", value_name = "topic", long_help = "Add topic to the targetted list")]
    add: Option<String>,
    /// List
    #[arg(short = 'l', long = "list", long_help = "List entries in the targtted list")]
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

    /*
        In order:
        Processes the deletion of files
        Processes creation of files
        Processes the list's path and length, can be modified with the --target flag
     */
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
            /* 
                Default behaviour of program without any flags
                However, the --target flag does not count towards
                this and will activate this

                Plan to add functionality to not make this a default
             */
            if !args.list && !args.del.is_some() 
            && !args.read.is_some() 
            && !args.create.is_some() 
            && !args.dell.is_some() 
            && !args.listf {
                let line_count = list_r.lines().count();
                // really inefficient ^
                default_rand(line_count, &path);
            }
        },
    }


    // Parsing the delete flag
    match args.del {
        Some(ln) => {
            println!("────────────────────────");
            delete_ln(ln, &path);
        },
        _ => {},
    }

    // Read the line at index
    match args.read {
        Some(ln) => {
            println!("────────────────────────");
            let (output, success) = show(ln, &path);
            if success {
                println!("{}", output);
                rprompt(ln, &path);

            } else {
                println!("ERROR! {output}");
            }
        },
        _ => {},
    }

    println!("────────────────────────");

}
