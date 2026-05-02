use std::process::Command;
pub fn execute_external_command(command: &str, args: &[String]) -> std::process::Child {
    Command::new(command)
        .args(args)
        .spawn()
        .expect("Failed to execute!")
}