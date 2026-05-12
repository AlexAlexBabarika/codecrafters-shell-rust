use crate::commands::builtin_command_names::BuiltinCommandName;
use crate::commands::command_props::CommandProps;
use crate::commands::command_result::CommandResult;
use crate::commands::shell_command::Execute;
use crate::commands::shell_command::ShellCommandError;
use crate::util::check_arguments_length::check_arguments_length;

pub struct COMPLETE {
    #[allow(dead_code)]
    pub props: CommandProps,
}

impl COMPLETE {
    pub fn new() -> Self {
        COMPLETE {
            props: CommandProps {
                name: BuiltinCommandName::Complete,
                max_args: 2,
            },
        }
    }
}

impl Execute for COMPLETE {
    fn execute(&self, args: &[String]) -> Result<CommandResult, ShellCommandError> {
        check_arguments_length(args, &self.props)?;
        if args.first().map(|s| s.as_str()) == Some("-p") {
            if let Some(cmd_name) = args.get(1) {
                return Ok(CommandResult {
                    message: Some(
                        format!(
                            "{}: {}: no completion specification",
                            self.props.name, cmd_name
                        )
                        .to_string(),
                    ),
                });
            }
        }

        Ok(CommandResult { message: None })
    }
}
