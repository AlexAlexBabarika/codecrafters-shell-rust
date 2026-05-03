use crate::commands::command_registry::get_builtin_command_from_str;
use crate::commands::shell_command::ShellCommand;

pub fn is_shell_builtin(command_name: &str) -> (bool, Option<Box<dyn ShellCommand + Send + Sync>>) {
    if let Ok(cmd) = get_builtin_command_from_str(command_name) {
        (true, Some(cmd))
    } else {
        (false, None)
    }
}
