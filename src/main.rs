use parser::parse::parse_input;

use crate::util::execute_external_command::{execute_external_command, execute_external_command_capture_stdout};
use crate::util::is_shell_builtin::is_shell_builtin;
use crate::util::write_to_file::write_to_file;

use std::io::{Write, stdin, stdout};

mod commands;
mod parser;
mod util;

fn execute_command(args: Vec<String>) {
    match &args[..] {
        [cmd, args @ .., op, path] if matches!(op.as_str(), ">" | "1>") => {
            let content = execute_external_command_capture_stdout(cmd, args);
            if let Ok(output) = content {
                if let Err(e) = write_to_file(path, &output) {
                    println!("Error writing to file: {}", e);
                }
            }
        }
        [cmd, tail @ ..] => {
            if let Some(builtin) = is_shell_builtin(cmd) {
                match builtin.execute(tail) {
                    Ok(result) => {
                        if let Some(msg) = result.message {
                            println!("{}", msg);
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            } else {
                execute_external_command(cmd, tail).map_err(|e| println!("{}", e)).unwrap();
            }
        }
        [] => {}
    }
}

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

        execute_command(args);

        input.clear();
    }
}
