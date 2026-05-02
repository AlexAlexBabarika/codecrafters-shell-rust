use crate::commands::definition::command_result::CommandResult;
use crate::commands::command_names::CommandName;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellCommandError {
    #[error("{0}: not found")]
    NotFoundError(String),

    #[error("{comm}: failed to execute.\nReason: {reason}")]
    #[allow(dead_code)]
    FailedToExecute{
        comm: CommandName,
        reason: String
    },

    #[error("{comm}: too many arguments have been provided. The limit is {max_args} arguments")]
    TooManyArgs{
        comm: CommandName,
        max_args: usize
    },

    #[error("{comm}: no arguments have been provided. You can provide {max_args} argument(s)")]
    NoArgs{
        comm: CommandName,
        max_args: usize
    }
}

pub trait ShellCommand {
    fn execute(&self, args: &[String]) -> Result<Option<CommandResult>, ShellCommandError>;
}