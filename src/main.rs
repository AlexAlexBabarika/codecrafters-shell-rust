use std::io::{Write, stdin, stdout};

fn main() {
    let mut input = String::new();
    print!("$ ");
    let _ = stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    if let Some('\n') = input.chars().next_back() {
        input.pop();
        println!("{}: command not found", input)
    }
    
    stdout().flush().unwrap();
}
