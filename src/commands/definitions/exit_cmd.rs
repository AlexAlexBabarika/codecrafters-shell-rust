use crate::commands::command_names::CommandName;
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
                name: CommandName::Exit,
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
