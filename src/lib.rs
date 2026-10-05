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
    Ok(())
}
