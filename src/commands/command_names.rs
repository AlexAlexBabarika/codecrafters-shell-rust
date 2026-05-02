use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandName {
    Echo,
    Type,
    Exit,
}

impl CommandName {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Echo => "echo",
            Self::Type => "type",
            Self::Exit => "exit",
        }
    }
}

impl fmt::Display for CommandName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownCommandName;

impl FromStr for CommandName {
    type Err = UnknownCommandName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Self::Echo),
            "type" => Ok(Self::Type),
            "exit" => Ok(Self::Exit),
            _ => Err(UnknownCommandName),
        }
    }
}
