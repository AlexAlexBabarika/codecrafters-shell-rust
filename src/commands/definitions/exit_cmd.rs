use crate::commands::builtin_command_names::BuiltinCommandName;
use crate::commands::command_props::CommandProps;
use crate::commands::command_result::CommandResult;
use crate::commands::shell_command::Execute;
use crate::commands::shell_command::ShellCommandError;

pub struct EXIT {
    #[allow(dead_code)]
    pub props: CommandProps,
}

impl EXIT {
    pub fn new() -> Self {
        EXIT {
            props: CommandProps {
                name: BuiltinCommandName::Exit,
                max_args: 0,
            },
        }
    }
}

impl Execute for EXIT {
    fn execute(&self, _args: &[String]) -> Result<CommandResult, ShellCommandError> {
        std::process::exit(0);
    }
}
