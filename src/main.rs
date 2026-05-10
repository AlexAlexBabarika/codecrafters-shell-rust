use parser::parse::parse_input;
use shell::execute::execute_command;
use shell::shell_helper::ShellHelper;
use rustyline::{Editor, Result};

mod commands;
mod parser;
mod shell;
mod util;

fn main() -> Result<()> {
    let config = rustyline::Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .build();
    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(ShellHelper));

    loop {
        match rl.readline("$ ") {
            Ok(line) => {
                let line = line.trim();
                match parse_input(&mut line.to_string()) {
                    Ok(args) => execute_command(args),
                    Err(e) => println!("{}", e),
                }
            }
            Err(_) => break,
        }
    }
    Ok(())
}
