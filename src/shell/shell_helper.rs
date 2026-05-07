use crate::commands::builtin_command_names::get_builtin_command_names;

use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};

pub struct ShellHelper;

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let builtins: Vec<&'static str> = get_builtin_command_names();
        let prefix = &line[..pos];

        let matches: Vec<Pair> = builtins
            .iter()
            .filter(|b| b.starts_with(prefix))
            .map(|b| Pair {
                display: b.to_string(),
                replacement: format!("{} ", b),
            })
            .collect();

        Ok((0, matches))
    }
}

impl Hinter for ShellHelper {
    type Hint = String;
}
impl Highlighter for ShellHelper {}
impl Validator for ShellHelper {}
impl Helper for ShellHelper {}
