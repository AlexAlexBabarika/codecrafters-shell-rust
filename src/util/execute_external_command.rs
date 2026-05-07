use crate::commands::shell_command::ShellCommandError;
use std::io;
use std::process::{Command, Stdio};

fn map_spawn_error(command: &str, e: io::Error) -> ShellCommandError {
    if e.kind() == io::ErrorKind::NotFound {
        ShellCommandError::NotFoundError(command.to_string())
    } else {
        ShellCommandError::ExternalCommandError(e.to_string())
    }
}

pub fn execute_external_command(command: &str, args: &[String]) -> Result<(), ShellCommandError> {
    Command::new(command)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| map_spawn_error(command, e))?;

    Ok(())
}

pub fn execute_external_command_capture_stdout(
    command: &str,
    args: &[String],
) -> Result<String, ShellCommandError> {
    let output = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| map_spawn_error(command, e))?;

    String::from_utf8(output.stdout)
        .map_err(|e| ShellCommandError::ExternalCommandError(e.to_string()))
}
