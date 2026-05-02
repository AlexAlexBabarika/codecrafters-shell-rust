use std::io::{stdin, stdout, Write};
use crate::commands::command_registry::get_command_from_str;
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
            match get_command_from_str(&args[0]) {
                Ok(cmd) => {
                    let command_args = &args[1..];
                    match cmd.execute(command_args) {
                        Ok(Some(result)) => println!("{}", result),
                        Ok(None) => print!("The command returned None"),
                        Err(e) => println!("{}", e)
                    }
                },
                Err(e) => println!("{}", e)
            }
            
            // if !(COMMAND_REGISTRY.contains_key(&args[0])) {
            //     println!("{}: command not found", input);
            // } else {
            //     let command = &COMMAND_REGISTRY[&args[0]];
            //     let command_args = &args[1..];

            //     match command.execute(command_args) {
            //         Ok(Some(result)) => println!("{}", result),
            //         Ok(None) => {},
            //         Err(e) => println!("{}", e)
            //     }
            // }

            input.clear();
        }   
    }

    // println!("{0}", env::var_os("PATH").unwrap().into_string().unwrap());
    // println!("{:?}", find_it("ls"));
}
