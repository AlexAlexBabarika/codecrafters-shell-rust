use crate::commands::codes::CompletionCode;
use std::fmt;

#[derive(Debug)]
pub struct CommandResult {
    pub message: Option<String>,
    #[allow(dead_code)]
    pub code: CompletionCode,
}

impl fmt::Display for CommandResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.message.is_none() {
            return Ok(());
        }

        write!(f, "{}", self.message.as_ref().unwrap())
    }
}
