use std::process::Command;

pub fn execute_external_command(command: &str, args: &[String]) -> Result<(), std::io::Error> {
    let mut child = Command::new(command)
        .args(args)
        .spawn()?;

    child.wait()?;
    Ok(())
}