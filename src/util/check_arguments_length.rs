use crate::commands::definition::command_props::CommandProps;
use crate::commands::definition::shell_command::ShellCommandError;

pub fn check_arguments_length(args: &[String], cmd_props: &CommandProps) -> Result<(), ShellCommandError> {
    if args.is_empty() || args[0].is_empty() {
        return Err(ShellCommandError::NoArgs { comm: (cmd_props.name.to_string()), max_args: (cmd_props.max_args) })
    }

    if args.len() > cmd_props.max_args {
        return Err(ShellCommandError::TooManyArgs { comm: (cmd_props.name.to_string()), max_args: (cmd_props.max_args) })
    }

    Ok(())
}