use std::str::FromStr;

use crate::commands::command_definitions::*;
use crate::commands::command_names::CommandName;
use crate::commands::definition::shell_command::{ShellCommand, ShellCommandError};

pub fn get_builtin_command(name: CommandName) -> Result<Box<dyn ShellCommand + Send + Sync>, ShellCommandError> {
    let cmd: Box<dyn ShellCommand + Send + Sync> = match name {
        CommandName::Exit => Box::new(EXIT::new()),
        CommandName::Type => Box::new(TYPE::new()),
        CommandName::Echo => Box::new(ECHO::new()),
    };
    Ok(cmd)
}

pub fn get_builtin_command_from_str(raw: &str) -> Result<Box<dyn ShellCommand + Send + Sync>, ShellCommandError> {
    let name =
        CommandName::from_str(raw).map_err(|_| ShellCommandError::NotFoundError(raw.to_string()))?;
    get_builtin_command(name)
}
