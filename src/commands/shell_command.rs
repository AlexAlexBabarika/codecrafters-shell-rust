use crate::commands::command_names::CommandName;
use crate::commands::command_result::CommandResult;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellCommandError {
    #[error("{0}: command not found")]
    NotFoundError(String),

    #[error("{comm}: {reason}")]
    #[allow(dead_code)]
    FailedToExecute { comm: CommandName, reason: String },

    #[error("{comm}: too many arguments have been provided. The limit is {max_args} arguments")]
    TooManyArgs { comm: CommandName, max_args: usize },

    #[error("{comm}: no arguments have been provided. You can provide {max_args} argument(s)")]
    NoArgs { comm: CommandName, max_args: usize },

    #[error("{0}")]
    ExternalCommandError(String),
}

pub trait Execute {
    fn execute(&self, args: &[String]) -> Result<CommandResult, ShellCommandError>;
}

pub trait Fail {
    fn fail(&self, reason: String) -> ShellCommandError;
}
