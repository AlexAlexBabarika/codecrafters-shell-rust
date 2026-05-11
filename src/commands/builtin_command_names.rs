use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinCommandName {
    Echo,
    Type,
    Exit,
    Pwd,
    Cd,
}

impl BuiltinCommandName {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Echo => "echo",
            Self::Type => "type",
            Self::Exit => "exit",
            Self::Pwd => "pwd",
            Self::Cd => "cd",
        }
    }
}

pub fn get_builtin_command_names(add_trailing_whitespace: bool) -> Vec<String> {
    if add_trailing_whitespace {
        vec![
            format!("{} ", BuiltinCommandName::Echo.as_str()),
            format!("{} ", BuiltinCommandName::Type.as_str()),
            format!("{} ", BuiltinCommandName::Exit.as_str()),
            format!("{} ", BuiltinCommandName::Pwd.as_str()),
            format!("{} ", BuiltinCommandName::Cd.as_str()),
        ]
    } else {
        vec![
            BuiltinCommandName::Echo.as_str().to_string(),
            BuiltinCommandName::Type.as_str().to_string(),
            BuiltinCommandName::Exit.as_str().to_string(),
            BuiltinCommandName::Pwd.as_str().to_string(),
            BuiltinCommandName::Cd.as_str().to_string(),
        ]
    }
}

impl fmt::Display for BuiltinCommandName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownCommandName;

impl FromStr for BuiltinCommandName {
    type Err = UnknownCommandName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Self::Echo),
            "type" => Ok(Self::Type),
            "exit" => Ok(Self::Exit),
            "pwd" => Ok(Self::Pwd),
            "cd" => Ok(Self::Cd),
            _ => Err(UnknownCommandName),
        }
    }
}
