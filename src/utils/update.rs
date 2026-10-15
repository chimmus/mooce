use std::{fs::{File}, io::Write};

pub fn write_list(topic: String, path: &String)  -> std::io::Result<()> {
    let mut list_w = File::options()
        .append(true)
        .create(true)
        .open(path)?;
    writeln!(&mut list_w, "{topic}")?;
    println!("Added {topic}");
    Ok(())
}
