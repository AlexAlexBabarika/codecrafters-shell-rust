use crate::parser::parse_error::ParseError;

pub fn parse_input(buff: &mut String) -> Result<Vec<String>, ParseError> {
    if let Some('\n') = buff.chars().next_back() {
        buff.pop();
    }
    
    if buff.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    
    let mut args: Vec<String> = Vec::new();
    let mut chars = buff.chars().peekable();
    let mut curr_arg: String = String::new();
    let mut is_single_quoted: bool = false;

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' if !is_single_quoted => {
                if !curr_arg.is_empty() {
                    args.push(curr_arg.clone());
                    curr_arg.clear();
                }
                chars.next();
            },
            '\'' => {
                is_single_quoted = !is_single_quoted;
                chars.next();
            },
            _ => {
                curr_arg.push(c);
                chars.next();
            }
        }
    }

    if is_single_quoted {
        return Err(ParseError::UnclosedSingleQuote);
    }

    if !curr_arg.is_empty() {
        args.push(curr_arg.clone());
    }
    
    Ok(args)
}