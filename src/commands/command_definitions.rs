use crate::commands::definition::command_props::CommandProps;
use crate::commands::definition::shell_command::ShellCommand;
use crate::commands::codes::CompletionCode;
use crate::commands::command_registry::COMMAND_REGISTRY;

// exit
pub struct EXIT {
    pub props: CommandProps
}

impl EXIT {
    pub fn new() -> Self {
        EXIT {
            props: CommandProps {
                name: "exit".to_string(),
                max_args: 0
            }
        }
    }
}

impl ShellCommand for EXIT {
    fn execute(&self, _args: &[String]) -> CompletionCode {
        std::process::exit(0);
    }
}

// echo 
pub struct ECHO {
    pub props: CommandProps
}

impl ECHO {
    pub fn new() -> Self {
        ECHO {
            props: CommandProps {
                name: "echo".to_string(),
                max_args: 64
            }
        }
    }
}

impl ShellCommand for ECHO {
    fn execute(&self, _args: &[String]) -> CompletionCode {
        if _args.len() > self.props.max_args {
            println!("echo: too many arguments (max {})", self.props.max_args);
            return CompletionCode::Fail;
        }
        
        println!("{}", _args.join(" "));
        CompletionCode::Success
    }
}

// type
pub struct TYPE {
    pub props: CommandProps
}

impl TYPE {
    pub fn new() -> Self {
        TYPE {
            props: CommandProps {
                name: "type".to_string(),
                max_args: 1 
            }
        }
    }
}

impl ShellCommand for TYPE {
    fn execute(&self, args: &[String]) -> CompletionCode {
        if args.is_empty() || args[0].is_empty() {
            println!("No arguments provided.\nUsage: type [command]\nAllows the user to get the description of a provided command");
            return CompletionCode::Fail;
        }
        
        if COMMAND_REGISTRY.contains_key(&args[0]) {
            println!("{} is a shell builtin", args[0]);
            return CompletionCode::Success;
        } else {
            println!("{}: not found", args[0])
        }

        CompletionCode::Fail
    }
}
