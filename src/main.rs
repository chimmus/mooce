use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[command(version, about)]

struct Cli {
    #[arg(short = 'a', long = "add", value_name = "THING")]
    add: Option<String>,
}


fn main() {
    let args = Cli::parse();

    match args.add {
        Some(thing) => {todo!("add write to file")},
        None => {println!("{}", rand::thread_rng().gen_range(1..=100))},
    }
}
