mod commands;

use std::io::{Write, stdin, stdout};
use commands::command_registry::COMMAND_REGISTRY;

fn main() {
    let mut input = String::new();
    
    loop {
        print!("$ ");
        let _ = stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        if let Some('\n') = input.chars().next_back() {
            input.pop();
            if input.is_empty() {
                println!("Please provide input. Write 'exit' to exit the shell");
                continue;
            }

            let args: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
            
            if !(COMMAND_REGISTRY.contains_key(&args[0])) {
                println!("{}: command not found", input);
            } else {
                let command = &COMMAND_REGISTRY[&args[0]];
                let command_args = &args[1..];
                command.execute(command_args);
            }

            input.clear();
        }   
    }
}
