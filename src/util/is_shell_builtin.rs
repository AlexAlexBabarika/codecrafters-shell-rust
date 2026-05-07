use crate::commands::command_registry::get_builtin_command_from_str;
use crate::commands::shell_command::Execute;

pub fn is_shell_builtin(command_name: &str) -> Option<Box<dyn Execute + Send + Sync>> {
    if let Ok(cmd) = get_builtin_command_from_str(command_name) {
        Some(cmd)
    } else {
        None
    }
}
