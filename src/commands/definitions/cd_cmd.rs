use crate::commands::shell_command::ShellCommand;
use crate::commands::shell_command::ShellCommandError;
use crate::commands::command_names::CommandName;
use crate::commands::command_props::CommandProps;
use crate::commands::command_result::CommandResult;
use crate::commands::codes::CompletionCode;
use crate::util::check_arguments_length::check_arguments_length;
use std::path::PathBuf;

pub struct CD {
    pub props: CommandProps,
}

impl CD {
    pub fn new() -> Self {
        CD {
            props: CommandProps {
                name: CommandName::Cd,
                max_args: 1,
            },
        }
    }
}

impl ShellCommand for CD {
    fn execute(&self, args: &[String]) -> Result<CommandResult, ShellCommandError> {
        check_arguments_length(args, &self.props)?;

        let subargs = args[0]
            .split("/")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        let current_path: PathBuf = match std::env::current_dir() {
            Err(_e) => {
                return Err(ShellCommandError::FailedToExecute {
                    comm: self.props.name,
                    reason: "Failed to get current directory".to_string(),
                });
            }
            Ok(path) => path,
        };
        let mut new_path = PathBuf::new();

        for subarg in subargs.iter() {
            if subarg.contains('~') {
                new_path = match std::env::home_dir() {
                    None => {
                        return Err(ShellCommandError::FailedToExecute {
                            comm: self.props.name,
                            reason: "Failed to get home directory".to_string(),
                        });
                    }
                    Some(home) => home,
                };
            } else if subarg.contains("..") {
                new_path.pop();
            } else if subarg.contains('.') {
                new_path = current_path.clone();
            } else {
                new_path.push(subarg);
            }
        }

        match std::env::set_current_dir(new_path) {
            Err(_e) => {
                return Err(ShellCommandError::FailedToExecute {
                    comm: self.props.name,
                    reason: String::new(),
                });
            }
            Ok(_) => {
                return Ok(CommandResult {
                    message: None,
                    code: CompletionCode::Success,
                });
            }
        }
    }
}
