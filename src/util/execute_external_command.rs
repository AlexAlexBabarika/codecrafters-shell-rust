use crate::commands::shell_command::ShellCommandError;
use std::process::{Command, Stdio};

pub fn execute_external_command(command: &str, args: &[String]) -> Result<(), ShellCommandError> {
    Command::new(command)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| ShellCommandError::ExternalCommandError(e.to_string()))?;

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
        .map_err(|e| ShellCommandError::ExternalCommandError(e.to_string()))?;

    String::from_utf8(output.stdout)
        .map_err(|e| ShellCommandError::ExternalCommandError(e.to_string()))
}
