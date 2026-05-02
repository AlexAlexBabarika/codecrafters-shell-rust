use crate::commands::definition::command_result::CommandResult;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellCommandError {
    #[error("{0}: not found")]
    NotFoundError(String),

    #[error("{0}: failed to execute")]
    #[allow(dead_code)]
    FailedToExecute(String),

    #[error("{comm}: too many arguments have been provided. The limit is {max_args} arguments")]
    TooManyArgs{
        comm: String,
        max_args: usize
    },

    #[error("{comm}: no arguments have been provided. You can provide {max_args} arguments")]
    NoArgs{
        comm: String,
        max_args: usize
    }
}

pub trait ShellCommand {
    fn execute(&self, args: &[String]) -> Result<Option<CommandResult>, ShellCommandError>;
}