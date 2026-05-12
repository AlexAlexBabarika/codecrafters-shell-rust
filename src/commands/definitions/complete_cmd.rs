use crate::commands::builtin_command_names::BuiltinCommandName;
use crate::commands::command_props::CommandProps;
use crate::commands::command_result::CommandResult;
use crate::commands::shell_command::Execute;
use crate::commands::shell_command::ShellCommandError;

pub struct COMPLETE {
    #[allow(dead_code)]
    pub props: CommandProps,
}

impl COMPLETE {
    pub fn new() -> Self {
        COMPLETE {
            props: CommandProps {
                name: BuiltinCommandName::Complete,
                max_args: 1,
            },
        }
    }
}

impl Execute for COMPLETE {
    fn execute(&self, _args: &[String]) -> Result<CommandResult, ShellCommandError> {
        Ok(CommandResult {
            message: Some("COMPLETE command is not implemented yet".to_string()),
        })
    }
}
