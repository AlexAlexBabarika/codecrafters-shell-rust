use crate::commands::command_definitions::{EXIT, ECHO};
use crate::commands::definition::shell_command::ShellCommand;
use std::collections::HashMap;
use std::sync::LazyLock;

pub static COMMAND_REGISTRY: LazyLock<HashMap<String, Box<dyn ShellCommand + Send + Sync>>> =
    LazyLock::new(|| {
        let mut registry: HashMap<String, Box<dyn ShellCommand + Send + Sync>> = HashMap::new();
        
        let exit_com: EXIT = EXIT::new();
        registry.insert(exit_com.props.name.clone(), Box::new(exit_com));

        let echo_com : ECHO = ECHO::new();
        registry.insert(echo_com.props.name.clone(), Box::new(echo_com));
        
        registry
    });
