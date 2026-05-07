use parser::parse::parse_input;
use shell::execute::execute_command;
use shell::shell_helper::ShellHelper;

use rustyline::history::DefaultHistory;
use rustyline::{Editor, Result};

mod commands;
mod parser;
mod shell;
mod util;

fn main() -> Result<()> {
    // let mut input = String::new();
    let mut rl = Editor::<ShellHelper, DefaultHistory>::new()?;
    rl.set_helper(Some(ShellHelper));

    loop {
        // print!("$ ");
        // let _ = stdout().flush().unwrap();
        // stdin().read_line(&mut input).unwrap();

        // match parse_input(&mut input) {
        //     Ok(args) => execute_command(args),
        //     Err(e) => println!("{}", e),
        // }

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
