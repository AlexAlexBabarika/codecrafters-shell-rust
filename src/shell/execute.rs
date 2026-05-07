use crate::util::execute_external_command::{
    execute_external_command, execute_external_command_capture_stderr,
    execute_external_command_capture_stdout,
};
use crate::util::is_shell_builtin::is_shell_builtin;
use crate::util::write_to_file::{append_to_file, write_to_file};

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

pub fn execute_command(args: Vec<String>) {
    match &args[..] {
        [cmd, mid @ .., op, path] if matches!(op.as_str(), ">" | "1>") => {
            match execute_external_command_capture_stdout(cmd, mid) {
                Ok(output) => write_to_file(path, &output)
                    .unwrap_or_else(|e| println!("Error writing to file: {}", e)),
                Err(e) => println!("{}", e),
            }
        }
        [cmd, mid @ .., op, path] if matches!(op.as_str(), "2>") => {
            match execute_external_command_capture_stderr(cmd, mid) {
                Ok(output) => write_to_file(path, &output)
                    .unwrap_or_else(|e| println!("Error writing to file: {}", e)),
                Err(e) => println!("{}", e),
            }
        }
        [cmd, mid @ .., op, path] if matches!(op.as_str(), ">>" | "1>>") => {
            match execute_external_command_capture_stdout(cmd, mid) {
                Ok(output) => append_to_file(path, &output)
                    .unwrap_or_else(|e| println!("Error writing to file: {}", e)),
                Err(e) => println!("{}", e),
            }
        }
        [cmd, mid @ .., op, path] if matches!(op.as_str(), "2>>") => {
            match execute_external_command_capture_stderr(cmd, mid) {
                Ok(output) => append_to_file(path, &output)
                    .unwrap_or_else(|e| println!("Error writing to file: {}", e)),
                Err(e) => println!("{}", e),
            }
        }
        [cmd, tail @ ..] => run_builtin_or_external(cmd, tail),
        [] => {}
    }
}
