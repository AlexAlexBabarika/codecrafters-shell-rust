use crate::commands::builtin_command_names::BuiltinCommandName;
use crate::commands::command_props::CommandProps;
use crate::commands::command_result::CommandResult;
use crate::commands::shell_command::{Execute, ShellCommandError};
use crate::util::check_arguments_length::check_arguments_length;

pub struct ECHO {
    pub props: CommandProps,
}

impl ECHO {
    pub fn new() -> Self {
        ECHO {
            props: CommandProps {
                name: BuiltinCommandName::Echo,
                max_args: 64,
            },
        }
    }
}

impl Execute for ECHO {
    fn execute(&self, args: &[String]) -> Result<CommandResult, ShellCommandError> {
        check_arguments_length(args, &self.props)?;

        let result = format!("{}", args.join(" "));

        Ok(CommandResult {
            message: Some(result),
        })
    }
}
