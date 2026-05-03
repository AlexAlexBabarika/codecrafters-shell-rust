use crate::commands::shell_command::ShellCommand;
use crate::commands::shell_command::ShellCommandError;
use crate::commands::command_names::CommandName;
use crate::commands::command_props::CommandProps;
use crate::commands::command_result::CommandResult;
use crate::commands::codes::CompletionCode;
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

impl ShellCommand for PWD {
    fn execute(&self, args: &[String]) -> Result<CommandResult, ShellCommandError> {
        check_arguments_length(args, &self.props)?;

        match std::env::current_dir() {
            Err(e) => {
                return Err(ShellCommandError::FailedToExecute {
                    comm: self.props.name,
                    reason: format!("Failed to get current directory. Reason: {}", e),
                });
            }
            Ok(path) => {
                if let Some(path_str) = path.to_str() {
                    return Ok(CommandResult {
                        message: Some(path_str.to_string()),
                        code: CompletionCode::Success,
                    });
                } else {
                    return Err(ShellCommandError::FailedToExecute {
                        comm: self.props.name,
                        reason: "Failed to convert path to string".to_string(),
                    });
                }
            }
        }
    }
}
