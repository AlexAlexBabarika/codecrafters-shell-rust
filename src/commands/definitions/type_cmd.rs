use crate::commands::command_names::CommandName;
use crate::commands::command_props::CommandProps;
use crate::commands::command_result::CommandResult;
use crate::commands::shell_command::Execute;
use crate::commands::shell_command::ShellCommandError;
use crate::util::check_arguments_length::check_arguments_length;

pub struct TYPE {
    pub props: CommandProps,
}

impl TYPE {
    pub fn new() -> Self {
        TYPE {
            props: CommandProps {
                name: CommandName::Type,
                max_args: 1,
            },
        }
    }
}

impl Execute for TYPE {
    fn execute(&self, args: &[String]) -> Result<CommandResult, ShellCommandError> {
        check_arguments_length(args, &self.props)?;

        if crate::util::is_shell_builtin::is_shell_builtin(&args[0]).0 {
            return Ok(CommandResult {
                message: Some(format!("{} is a shell builtin", args[0])),
            });
        }

        if let Some(exe) = crate::util::find_executable::find_executable(&args[0]) {
            return Ok(CommandResult { message: Some(exe) });
        }

        return Err(ShellCommandError::NotFoundError(args[0].to_string()));
    }
}
