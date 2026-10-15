use std;

pub fn remove(line_i: usize, path: &String) -> std::io::Result<()> {
    let list_w = std::fs::read_to_string(&path)?;
    let mut lines: Vec<String> = list_w.lines().map(String::from).collect();
    if line_i < lines.len() {
        lines.remove(line_i);
    }
    let mut out = lines.join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    std::fs::write(&path, out)?;

    Ok(())
}

pub fn delete_ln(line_number: usize, list_path: &String) {
    // This delete is more expressive
    let list_r = std::fs::read_to_string(&list_path).expect("File requested was not found");
    let line_count = list_r.lines().count();
    if line_number > (line_count - 1) {
        println!("{line_number} does not exist!");
    } else {
        remove(line_number, &list_path).expect("Error occured while removing line");
        println!("[{line_number}] has been removed"); // Not sure about how .expect works, 
    }
}
