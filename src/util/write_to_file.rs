use std::fs::File;
use std::io::prelude::*;

pub fn write_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn append_to_file(path: &str, content: &str) -> std::io::Result<()> {
        match File::options().append(true).open(path) {
        Ok(mut file) => {
            file.write_all(content.as_bytes())?;
            Ok(())
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => write_to_file(path, content),
            _ => Err(e),
        },
    }