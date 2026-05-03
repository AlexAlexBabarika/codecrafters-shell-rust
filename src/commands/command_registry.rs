use std::str::FromStr;

use crate::commands::definitions::*;
use crate::commands::command_names::CommandName;
use crate::commands::shell_command::{ShellCommand, ShellCommandError};

pub fn get_builtin_command(
    name: CommandName,
) -> Result<Box<dyn ShellCommand + Send + Sync>, ShellCommandError> {
    let cmd: Box<dyn ShellCommand + Send + Sync> = match name {
        CommandName::Exit => Box::new(exit_cmd::EXIT::new()),
        CommandName::Type => Box::new(type_cmd::TYPE::new()),
        CommandName::Echo => Box::new(echo_cmd::ECHO::new()),
        CommandName::Pwd => Box::new(pwd_cmd::PWD::new()),
        CommandName::Cd => Box::new(cd_cmd::CD::new()),
    };
    Ok(cmd)
}

pub fn get_builtin_command_from_str(
    raw: &str,
) -> Result<Box<dyn ShellCommand + Send + Sync>, ShellCommandError> {
    let name = CommandName::from_str(raw)
        .map_err(|_| ShellCommandError::NotFoundError(raw.to_string()))?;
    get_builtin_command(name)
}
