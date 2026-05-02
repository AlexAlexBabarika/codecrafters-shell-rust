use crate::commands::definition::command_names::CommandName;

pub struct CommandProps {
    pub name: CommandName,
    pub max_args: usize
}
