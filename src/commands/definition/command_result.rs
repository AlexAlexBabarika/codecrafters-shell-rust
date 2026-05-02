use crate::commands::codes::CompletionCode;
use std::fmt;

#[derive(Debug)]
pub struct CommandResult {
    pub message: String,
    #[allow(dead_code)]
    pub code: CompletionCode
}

impl fmt::Display for CommandResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
