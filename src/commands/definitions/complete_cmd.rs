use crate::commands::builtin_command_names::BuiltinCommandName;
use crate::commands::command_props::CommandProps;
use crate::commands::command_result::CommandResult;
use crate::commands::shell_command::Execute;
use crate::commands::shell_command::ShellCommandError;
use crate::util::check_arguments_length::check_arguments_length;

use crate::completion_registry::completion_registry::{get_completion, register_completion};

pub struct COMPLETE {
    #[allow(dead_code)]
    pub props: CommandProps,
}

impl COMPLETE {
    pub fn new() -> Self {
        COMPLETE {
            props: CommandProps {
                name: BuiltinCommandName::Complete,
                max_args: 3,
            },
        }
    }
}

impl Execute for COMPLETE {
    fn execute(&self, args: &[String]) -> Result<CommandResult, ShellCommandError> {
        check_arguments_length(args, &self.props)?;
        match args.first().map(|s| s.as_str()) {
            Some("-p") => {
                let cmd_name = args
                    .get(1)
                    .ok_or_else(|| ShellCommandError::InvalidArguments {
                        comm: self.props.name,
                        message: "missing command name argument for -p option".to_string(),
                    })?;

                return match get_completion(cmd_name) {
                    Some(spec) => Ok(CommandResult {
                        message: Some(format!("{} -C '{}' {}", self.props.name, spec, cmd_name)),
                    }),
                    None => Ok(CommandResult {
                        message: Some(format!(
                            "{}: {}: no completion specification",
                            self.props.name, cmd_name
                        )),
                    }),
                };
            }
            Some("-C") => {
                let completion_spec =
                    args.get(1)
                        .ok_or_else(|| ShellCommandError::InvalidArguments {
                            comm: self.props.name,
                            message: "missing completion specification argument for -C option"
                                .to_string(),
                        })?;

                let cmd_name = args
                    .get(2)
                    .ok_or_else(|| ShellCommandError::InvalidArguments {
                        comm: self.props.name,
                        message: "missing command name argument for -p option".to_string(),
                    })?;

                register_completion(cmd_name, completion_spec);
                return Ok(CommandResult { message: None });
            }
            _ => {}
        }

        Ok(CommandResult { message: None })
    }
}
