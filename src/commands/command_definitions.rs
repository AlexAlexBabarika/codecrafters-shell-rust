use crate::commands::definition::command_props::CommandProps;
use crate::commands::definition::shell_command::ShellCommand;
use crate::commands::codes::CompletionCode;

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
