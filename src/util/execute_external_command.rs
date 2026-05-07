use crate::commands::shell_command::ShellCommandError;
use std::io;
use std::process::{Command, Stdio};

pub struct ExternalCommandResult {
    pub stdout: String,
    pub stderr: String,
    // pub exit_code: i32,
}

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
) -> Result<ExternalCommandResult, ShellCommandError> {
    let output = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| map_spawn_error(command, e))?;

    Ok(ExternalCommandResult {
        stdout: String::from_utf8(output.stdout)
            .map_err(|e| ShellCommandError::ExternalCommandError(e.to_string()))?,
        stderr: String::from_utf8(output.stderr)
            .map_err(|e| ShellCommandError::ExternalCommandError(e.to_string()))?,
        // exit_code: output.status.code().unwrap_or(1),
    })
}
