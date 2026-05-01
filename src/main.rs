use std::io::{Write, stdin, stdout};

fn main() {
    let mut input = String::new();
    let mut is_running: bool = true;
    while is_running {
        print!("$ ");
        let _ = stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        if let Some('\n') = input.chars().next_back() {
            input.pop();
            
            if input != "exit" {
                println!("{}: command not found", input);
            }
            else {
                is_running = false;
            }

            input.clear();
        }   
    }
}
