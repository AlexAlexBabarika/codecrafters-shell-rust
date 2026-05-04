use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Can't parse an input with an unclosed single quote")]
    UnclosedSingleQuote,
    #[error("Please provide input. Write 'exit' to exit the shell")]
    EmptyInput,
}
