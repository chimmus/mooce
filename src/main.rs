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

    match args.dell {
        Some(ref file) => {
            let check = format!("~/.config/mooce/{}", file);
            let path_check = shellexpand::tilde(&check).into_owned();
            if !mooce::exist(&path_check) {
                println!("The file didn't exist so we didn't need to do anything");
            } else {
                let success = match fs::remove_file(&path_check) {
                    Ok(()) => true,
                    Err(_) => false,
                };
                if success {
                    println!("File successfully deleted!");
                } else {
                    println!("Error occured")
                }
            }
        },
        None => {},
    }

    match args.create {
        Some(ref file) => {
            if file.contains(".") {
                println!("Unfortunately mooce wont allow this, we do not permit . inside of the list name");
            } else {
                let check = format!("~/.config/mooce/{}", file);
                let path_check = shellexpand::tilde(&check).into_owned();
                if mooce::exist(&path_check) {
                    println!("There seems to be a file there with the same name");
                } else {
                    let _ = fs::File::create(&path_check);
                    println!("File successfully created!");
                }
            }
        },
        None => {},
    }

    let (list_r, path) = match args.target {
        Some(ref file) => {
            let path = format!("~/.config/mooce/{}", file);
            let path_check = shellexpand::tilde(&path).into_owned();
            if mooce::exist(&path_check) {
                let list_r = fs::read_to_string(&path_check)
                    .expect("The file you requested was not found, please add a file there");
                println!("────────────────────────");
                println!("Targetting {file}");
                (list_r, path_check)
            } else {
                let list_path = shellexpand::tilde("~/.config/mooce/list").into_owned();
                let list_r = fs::read_to_string(&list_path)
                    .expect("There is no file at ~/.config/mooce called \"list\" , please add a file there");

                (list_r, path_check)
            }
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

    if args.listf {
        println!("────────────────────────");
        mooce::ls();
    }

    match args.add {
        Some(topic) => {
            println!("────────────────────────");
            let _ = mooce::write_to_list(format!("{topic}"), &path);
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
                    mooce::print_r(line_count, &path);
                }
            }
        },
    }

    match args.del {
        Some(ln) => {
            println!("────────────────────────");
            mooce::delete_ln(ln, &path);
        },
        _ => {},
    }

    match args.read {
        Some(ln) => {
            println!("────────────────────────");
            mooce::print(ln, &path);
        },
        _ => {},
    }

    println!("────────────────────────");

}
