use crate::commands::command_names::CommandName;
use crate::commands::command_props::CommandProps;
use crate::commands::command_result::CommandResult;
use crate::commands::shell_command::{Execute, Fail, ShellCommandError};
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

    fn build_path(
        &self,
        current_path: &PathBuf,
        args: Vec<&str>,
    ) -> Result<PathBuf, ShellCommandError> {
        let mut new_path = current_path.clone();

        for subarg in args.iter() {
            if subarg.contains('~') {
                new_path = match std::env::home_dir() {
                    None => {
                        return Err(self.fail("Failed to get home directory".to_string()));
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

        Ok(new_path)
    }
}

impl Fail for CD {
    fn fail(&self, reason: String) -> ShellCommandError {
        ShellCommandError::FailedToExecute {
            comm: self.props.name,
            reason,
        }
    }
}

impl Execute for CD {
    fn execute(&self, args: &[String]) -> Result<CommandResult, ShellCommandError> {
        check_arguments_length(args, &self.props)?;

        let subargs = args[0]
            .split("/")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        let current_path: PathBuf = match std::env::current_dir() {
            Err(_e) => {
                return Err(self.fail("Failed to get current directory".to_string()));
            }
            Ok(path) => path,
        };

        let new_path = self.build_path(&current_path, subargs)?;
        // std::env::set_current_dir(&new_path).map_err(|_e| {
        //     self.fail(format!(
        //         "Failed to change directory to {}",
        //         new_path.to_str().unwrap_or("")
        //     ))
        // })?;
        // 
        std::env::set_current_dir(&new_path).unwrap();
        Ok(CommandResult { message: (None) })
    }
}
