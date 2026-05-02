use std::io::{stdin, stdout, Write};
use crate::util::is_shell_builtin::is_shell_builtin;
use crate::util::execute_external_command::execute_external_command;
mod commands;
mod util;

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
            if let (true, Some(cmd)) = is_shell_builtin(&args[0]) {
                match cmd.execute(&args[1..]) {
                    Ok(Some(result)) => println!("{}", result),
                    Ok(None) => print!("The command returned None"),
                    Err(e) => println!("{}", e)
                }
            } 
            else {
                execute_external_command(&args[0], &args[1..])
                    .unwrap_or_else(|e| println!("Error executing external command: {}", e));
            }

            input.clear();
        }   
    }
}
