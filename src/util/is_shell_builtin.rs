use std::str::FromStr;
use crate::commands::command_names::CommandName;

pub fn is_shell_builtin(command_name: &str) -> bool {
    match CommandName::from_str(command_name) {
        Err(_e) => false,
        Ok(_) => true
    }
}