use std::fs::File;
use std::io::Write;
use std::fs;
use rand::Rng;
use std::io;

pub fn write_to_list(topic: String, path: &String)  -> std::io::Result<()> {
    let mut list_w = File::options()
        .append(true)
        .create(true)
        .open(path)?;
    writeln!(&mut list_w, "{topic}")?;
    println!("Added {topic}");
    Ok(())
}

pub fn remove_line(line_i: usize, path: &String) -> std::io::Result<()> {
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

pub fn print(ln: usize, path: &String) {
    let list_r = fs::read_to_string(&path).expect("File requested was not found");
    if ln > list_r.lines().count() {
        println!("{ln} does not exist!");
    } else {
        let line = list_r.lines().nth(ln);
        if let Some(e) = line {
            println!("[{ln}] {e}")
        }
        rprompt(ln, &path);
    }
}

pub fn print_r(line_count: usize, list_path: &String) {
    let list_r = fs::read_to_string(&list_path).expect("File requested was not found");

    let rand_line = rand::thread_rng().gen_range(0..line_count);
    let line = list_r.lines().nth(rand_line);
    if let Some(e) = line {
        // remove the some(" ") formatting from the text
        println!("{e}");
    }
    rprompt(rand_line, &list_path);
}

pub fn delete_ln(line_number: usize, list_path: &String) {
    let list_r = fs::read_to_string(&list_path).expect("File requested was not found");
    let line_count = list_r.lines().count();
    if line_number > (line_count - 1) {
        println!("{line_number} does not exist!");
    } else {
        remove_line(line_number, &list_path).expect("Error occured while removing line");
        println!("[{line_number}] has been removed"); // Not sure about how .expect works, 
    }
}

fn rprompt(ln: usize, path: &String) {
    println!("Complete this topic? (y/n)");
    let mut prompt = String::new();
    loop {
        io::stdin().read_line(&mut prompt).expect("Failed to understand");
        prompt = prompt.trim().to_lowercase();
        if prompt.contains("y") {
            remove_line(ln, &path).expect("An error occured");
            println!("Removed topic with id {ln}");
            break
        } else {
            println!("You can find this again, the id of this is {:?}", ln);
            break
        }
    }
}

pub fn exist(path: &String) -> bool {
    match fs::exists(path) {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => false,
    }
}

pub fn ls() {
    let path_c = shellexpand::tilde("~/.config/mooce/").into_owned();
    for path in fs::read_dir(&path_c).expect("an error occured") {
        let path_e = path.expect("Could not read entry");
        let path_c = path_e.file_name().to_string_lossy().into_owned();
        if !path_c.contains(".") {
            println!("{}", path_c);
        }
    }
}
