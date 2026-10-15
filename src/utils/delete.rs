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
