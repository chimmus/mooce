use std::fs::File;
use std::io::Write;
use std::fs;
use rand::Rng;
use std::io;

pub fn write_to_list(topic: String)  -> std::io::Result<()> {
    let path = shellexpand::tilde("~/.config/mooce/list").into_owned();
    let mut list_w = File::options()
        .append(true)
        .create(true)
        .open(path)?;
    writeln!(&mut list_w, "{topic}")?;
    println!("Added {topic}");
    Ok(())
}

pub fn remove_line(line_i: usize) -> std::io::Result<()> {
    let path = shellexpand::tilde("~/.config/mooce/list").into_owned();
    let list_w = fs::read_to_string(&path)?;
    let mut lines: Vec<String> = list_w.lines().map(String::from).collect();
    if line_i < lines.len() {
        lines.remove(line_i);
    }
    let mut out = lines.join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    fs::write(&path, out)?;

    Ok(())
}

pub fn print_r(line_count: usize) {
    let list_path = shellexpand::tilde("~/.config/mooce/list").into_owned();
    let list_r = fs::read_to_string(list_path).expect("There is no file at ~/.config/mooce called \"list\" , please add a file there");

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
            remove_line(rand_line).expect("An error occured");
            println!("Removed topic with id {rand_line}");
            break
        } else {
            println!("You can find this again, the id of this is {:?}", rand_line);
            break
        }
    }
}
