use crate::commands::codes::CompletionCode;

pub trait ShellCommand {
    fn execute(&self, args: &[String]) -> CompletionCode;
}