use parser::parse::parse_input;

use crate::util::execute_external_command::execute_external_command;
use crate::util::is_shell_builtin::is_shell_builtin;

use std::io::{Write, stdin, stdout};

mod commands;
mod util;
mod parser;

fn main() {
    let mut input: String = String::new();

    loop {
        print!("$ ");
        let _ = stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        let args: Vec<String> = match parse_input(&mut input) {
            Ok(args) => args,
            Err(e) => {
                println!("{}", e);
                input.clear();
                continue;
            }
        };

        if let (true, Some(cmd)) = is_shell_builtin(&args[0]) {
            match cmd.execute(&args[1..]) {
                Ok(result) => {
                    if let Some(msg) = result.message {
                        println!("{}", msg);
                    }
                }
                Err(e) => println!("{}", e),
            }
        } else {
            execute_external_command(&args[0], &args[1..])
                .unwrap_or_else(|e| println!("{}", e));
        }

        input.clear();
    }
}
