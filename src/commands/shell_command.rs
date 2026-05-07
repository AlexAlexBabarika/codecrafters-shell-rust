use crate::commands::builtin_command_names::BuiltinCommandName;
use crate::commands::command_result::CommandResult;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellCommandError {
    #[error("{0}: not found")]
    NotFoundError(String),

    #[error("{comm}: {reason}")]
    #[allow(dead_code)]
    FailedToExecute {
        comm: BuiltinCommandName,
        reason: String,
    },

    #[error("{comm}: too many arguments have been provided. The limit is {max_args} arguments")]
    TooManyArgs {
        comm: BuiltinCommandName,
        max_args: usize,
    },

    #[error("{comm}: no arguments have been provided. You can provide {max_args} argument(s)")]
    NoArgs {
        comm: BuiltinCommandName,
        max_args: usize,
    },

    #[error("{0}")]
    ExternalCommandError(String),
}

pub trait Execute {
    fn execute(&self, args: &[String]) -> Result<CommandResult, ShellCommandError>;
}

pub trait Fail {
    fn fail(&self, reason: String) -> ShellCommandError;
}
