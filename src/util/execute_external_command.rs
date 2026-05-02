use std::io::{self, ErrorKind};
use std::process::Command;

pub fn execute_external_command(command: &str, args: &[String]) -> Result<(), io::Error> {
    let mut child = Command::new(command).args(args).spawn().map_err(|e| {
        if e.kind() == ErrorKind::NotFound {
            io::Error::new(ErrorKind::NotFound, format!("{}: command not found", command))
        } else {
            e
        }
    })?;

    child.wait()?;
    Ok(())
}
