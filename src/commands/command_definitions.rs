use crate::commands::definition::command_props::CommandProps;
use crate::commands::definition::shell_command::ShellCommand;
use crate::commands::definition::shell_command::ShellCommandError;
use crate::commands::definition::command_result::CommandResult;
use crate::commands::command_names::CommandName;
use crate::commands::codes::CompletionCode;
use crate::util::check_arguments_length::check_arguments_length;

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
                message: Some(result),
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

        if crate::util::is_shell_builtin::is_shell_builtin(&args[0]).0 {
            return Ok(Some(
                CommandResult {
                    message: Some(format!("{} is a shell builtin", args[0])),
                    code: CompletionCode::Success
                }
            ))
        }

        if let Some(exe) = crate::util::find_executable::find_executable(&args[0]) {
            return Ok(Some(
                CommandResult {
                    message: Some(exe),
                    code: CompletionCode::Success
                }
            ))
        }
        
        return Err(ShellCommandError::NotFoundError(args[0].to_string()));
    }
}

// pwd
pub struct PWD {
    pub props: CommandProps
}

impl PWD {
    pub fn new() -> Self {
        PWD {
            props: CommandProps {
                name: CommandName::Pwd,
                max_args: 0 
            }
        }
    }
}

impl ShellCommand for PWD {
    fn execute(&self, args: &[String]) -> Result<Option<CommandResult>, ShellCommandError> {        
        check_arguments_length(args, &self.props)?;

        match std::env::current_dir() {
            Err(e) => {
                return Err(ShellCommandError::FailedToExecute {
                    comm: self.props.name,
                    reason: format!("Failed to get current directory. Reason: {}", e)
                });
            },
            Ok(path) => {
                if let Some(path_str) = path.to_str() {
                    return Ok(Some(
                        CommandResult {
                            message: Some(path_str.to_string()),
                            code: CompletionCode::Success
                        }
                    ))
                }
                else {
                    return Err(ShellCommandError::FailedToExecute {
                        comm: self.props.name,
                        reason: "Failed to convert path to string".to_string()
                    });
                }
            }
        }
    }
}

// cd
pub struct CD {
    pub props: CommandProps
}

impl CD {
    pub fn new() -> Self {
        CD {
            props: CommandProps {
                name: CommandName::Cd,
                max_args: 1
            }
        }
    }
}

impl ShellCommand for CD {
    fn execute(&self, args: &[String]) -> Result<Option<CommandResult>, ShellCommandError> {        
        check_arguments_length(args, &self.props)?;


        if let Err(e) = std::env::set_current_dir(std::path::Path::new(&args[0])) {
            return Err(ShellCommandError::FailedToExecute {
                comm: self.props.name,
                reason: format!("Failed to change directory.\n{}", e)
            });
        }
        else {
            return Ok(Some(
                CommandResult {
                    message: None,
                    code: CompletionCode::Success
                }
            ))
        }
    }
}
