use parser::parse::parse_input;

use crate::util::execute_external_command::{execute_external_command, execute_external_command_capture_stdout};
use crate::util::is_shell_builtin::is_shell_builtin;
use crate::util::write_to_file::write_to_file;

use std::io::{stdin, stdout, Write};

mod commands;
mod parser;
mod util;

fn run_redirect(cmd: &str, mid: &[String], path: &str) {
    match execute_external_command_capture_stdout(cmd, mid) {
        Ok(output) => write_to_file(path, &output)
            .unwrap_or_else(|e| println!("Error writing to file: {}", e)),
        Err(e) => println!("{}", e),
    }
}

fn run_builtin_or_external(cmd: &str, tail: &[String]) {
    match is_shell_builtin(cmd) {
        Some(builtin) => match builtin.execute(tail) {
            Ok(result) => {
                if let Some(msg) = result.message {
                    println!("{}", msg);
                }
            }
            Err(e) => println!("{}", e),
        },
        None => execute_external_command(cmd, tail).unwrap_or_else(|e| println!("{}", e)),
    }
}

fn execute_command(args: Vec<String>) {
    match &args[..] {
        [cmd, mid @ .., op, path] if matches!(op.as_str(), ">" | "1>") => {
            run_redirect(cmd, mid, path);
        }
        [cmd, tail @ ..] => run_builtin_or_external(cmd, tail),
        [] => {}
    }
}

fn main() {
    let mut input = String::new();

    loop {
        print!("$ ");
        let _ = stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        match parse_input(&mut input) {
            Ok(args) => execute_command(args),
            Err(e) => println!("{}", e),
        }

        input.clear();
    }
}
