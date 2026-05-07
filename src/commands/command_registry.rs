use std::str::FromStr;

use crate::commands::builtin_command_names::BuiltinCommandName;
use crate::commands::definitions::*;
use crate::commands::shell_command::{Execute, ShellCommandError};

pub fn get_builtin_command(
    name: BuiltinCommandName,
) -> Result<Box<dyn Execute + Send + Sync>, ShellCommandError> {
    let cmd: Box<dyn Execute + Send + Sync> = match name {
        BuiltinCommandName::Exit => Box::new(exit_cmd::EXIT::new()),
        BuiltinCommandName::Type => Box::new(type_cmd::TYPE::new()),
        BuiltinCommandName::Echo => Box::new(echo_cmd::ECHO::new()),
        BuiltinCommandName::Pwd => Box::new(pwd_cmd::PWD::new()),
        BuiltinCommandName::Cd => Box::new(cd_cmd::CD::new()),
    };
    Ok(cmd)
}

pub fn get_builtin_command_from_str(
    raw: &str,
) -> Result<Box<dyn Execute + Send + Sync>, ShellCommandError> {
    let name = BuiltinCommandName::from_str(raw)
        .map_err(|_| ShellCommandError::NotFoundError(raw.to_string()))?;
    get_builtin_command(name)
}
