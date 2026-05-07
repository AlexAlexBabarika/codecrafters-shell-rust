use crate::commands::builtin_command_names::BuiltinCommandName;

pub struct CommandProps {
    pub name: BuiltinCommandName,
    pub max_args: usize,
}
