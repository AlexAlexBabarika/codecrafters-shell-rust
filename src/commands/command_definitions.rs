use crate::commands::definition::command_props::CommandProps;
use crate::commands::definition::shell_command::ShellCommand;
use crate::commands::definition::shell_command::ShellCommandError;
use crate::commands::definition::command_result::CommandResult;
use crate::commands::command_names::CommandName;
use crate::commands::codes::CompletionCode;
use crate::util::check_arguments_length::check_arguments_length;
use crate::util::is_shell_builtin::is_shell_builtin;

// exit
pub struct EXIT {
    #[allow(dead_code)]
    pub props: CommandProps
}

impl EXIT {
    pub fn new() -> Self {
        EXIT {
            props: CommandProps {
                name: CommandName::Exit,
                max_args: 0
            }
        }
    }
}

impl ShellCommand for EXIT {
    fn execute(&self, _args: &[String]) -> Result<Option<CommandResult>, ShellCommandError> {
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
                name: CommandName::Echo,
                max_args: 64
            }
        }
    }
}

impl ShellCommand for ECHO {
    fn execute(&self, args: &[String]) -> Result<Option<CommandResult>, ShellCommandError> {
        check_arguments_length(args, &self.props)?;

        let result = format!("{}", args.join(" "));
        
        Ok(Some(
            CommandResult {
                message: result,
                code: CompletionCode::Success
            }
        ))
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
                name: CommandName::Type,
                max_args: 1 
            }
        }
    }
}

impl ShellCommand for TYPE {
    fn execute(&self, args: &[String]) -> Result<Option<CommandResult>, ShellCommandError> {        
        check_arguments_length(args, &self.props)?;

        if is_shell_builtin(&args[0]).0 {
            return Ok(Some(
                CommandResult {
                    message: format!("{} is a shell builtin", args[0]),
                    code: CompletionCode::Success
                }
            ))
        }

        if let Some(exe) = crate::util::find_executable::find_executable(&args[0]) {
            return Ok(Some(
                CommandResult {
                    message: exe,
                    code: CompletionCode::Success
                }
            ))
        }
        
        return Err(ShellCommandError::NotFoundError(args[0].to_string()));
    }
}
