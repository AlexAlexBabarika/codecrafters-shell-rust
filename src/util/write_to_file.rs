use std::fs::File;
use std::io::prelude::*;

pub fn write_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn append_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::options().append(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
