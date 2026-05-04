use crate::commands::command_names::CommandName;
use crate::commands::command_props::CommandProps;
use crate::commands::command_result::CommandResult;
use crate::commands::shell_command::{Execute, Fail, ShellCommandError};
use crate::util::check_arguments_length::check_arguments_length;

pub struct PWD {
    pub props: CommandProps,
}

impl PWD {
    pub fn new() -> Self {
        PWD {
            props: CommandProps {
                name: CommandName::Pwd,
                max_args: 0,
            },
        }
    }
}

impl Fail for PWD {
    fn fail(&self, reason: String) -> ShellCommandError {
        ShellCommandError::FailedToExecute {
            comm: self.props.name,
            reason,
        }
    }
}

impl Execute for PWD {
    fn execute(&self, args: &[String]) -> Result<CommandResult, ShellCommandError> {
        check_arguments_length(args, &self.props)?;

        match std::env::current_dir() {
            Err(e) => {
                return Err(self.fail(format!("Failed to get current directory. Reason: {}", e)));
            }
            Ok(path) => {
                if let Some(path_str) = path.to_str() {
                    return Ok(CommandResult {
                        message: Some(path_str.to_string()),
                    });
                } else {
                    return Err(self.fail("Failed to convert path to string".to_string()));
                }
            }
        }
    }
}
