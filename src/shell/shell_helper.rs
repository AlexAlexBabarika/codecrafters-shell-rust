use crate::commands::builtin_command_names::get_builtin_command_names;

use crate::util::find_executable::get_path_executables;
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};

pub struct ShellHelper;

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let builtins: Vec<String> = get_builtin_command_names();
        let executables = get_path_executables().unwrap_or_default();
        let mut builtins_and_executables = builtins;
        builtins_and_executables.extend(executables);

        let prefix = &line[..pos];

        let matches: Vec<Pair> = builtins_and_executables
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
