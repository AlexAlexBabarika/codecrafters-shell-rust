use crate::parser::parse_error::ParseError;

pub fn parse_input(buff: &mut String) -> Result<Vec<String>, ParseError> {
    while matches!(buff.chars().next_back(), Some('\n' | '\r')) {
        buff.pop();
    }

    if buff.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let mut args = Vec::new();
    let mut chars = buff.chars().peekable();
    let mut curr_arg = String::new();
    let mut in_single = false;
    let mut in_double = false;

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' if !in_single && !in_double => {
                if !curr_arg.is_empty() {
                    args.push(std::mem::take(&mut curr_arg));
                }
                chars.next();
            }
            '\'' => {
                if in_double {
                    curr_arg.push('\'');
                } else {
                    in_single = !in_single;
                }
                chars.next();
            }
            '"' => {
                if in_single {
                    curr_arg.push('"');
                } else {
                    in_double = !in_double;
                }
                chars.next();
            }
            '\\' if !in_single => {
                chars.next();
                let Some(&next) = chars.peek() else {
                    if in_double {
                        return Err(ParseError::UnclosedDoubleQuote);
                    }
                    curr_arg.push('\\');
                    continue;
                };

                if in_double {
                    match next {
                        '"' | '\\' | '$' | '`' => {
                            curr_arg.push(next);
                            chars.next();
                        }
                        _ => {
                            curr_arg.push('\\');
                            continue;
                        }
                    }
                } else {
                    curr_arg.push(next);
                    chars.next();
                }
            }
            _ => {
                curr_arg.push(c);
                chars.next();
            }
        }
    }

    if in_single {
        return Err(ParseError::UnclosedSingleQuote);
    }
    if in_double {
        return Err(ParseError::UnclosedDoubleQuote);
    }
    if !curr_arg.is_empty() {
        args.push(curr_arg);
    }
    Ok(args)
}
