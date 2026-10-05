use std::fs::File;
use std::io::Write;
use std::fs;

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
