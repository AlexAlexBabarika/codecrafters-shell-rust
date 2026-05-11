use crate::shell::completer_cache::{completion_names_cached, current_dir_entries_cached};
use crate::util::path_utilities::get_matching_entries_in_directory;
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};

pub struct ShellHelper;

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let start = line[..pos]
            .rfind(char::is_whitespace)
            .map(|i| i + 1)
            .unwrap_or(0);
        let prefix = &line[start..pos];

        let names = if line[..start].trim().is_empty() {
            completion_names_cached()
        } else {
            if prefix.contains('/') {
                let split = prefix.rfind('/').unwrap() + 1;
                let (dir_part, file_prefix) = prefix.split_at(split);
                get_matching_entries_in_directory(dir_part, file_prefix).unwrap_or_default()
            } else {
                current_dir_entries_cached()
            }
        };

        let include_hidden = prefix.starts_with('.');
        let mut matches: Vec<Pair> = names
            .iter()
            .filter(|b| b.starts_with(prefix))
            .filter(|b| include_hidden || !b.starts_with('.'))
            .map(|b| Pair {
                display: b.clone(),
                replacement: format!("{}", b),
            })
            .collect();
        matches.sort_by(|a, b| a.display.cmp(&b.display));

        Ok((start, matches))
    }
}

impl Hinter for ShellHelper {
    type Hint = String;
}
impl Highlighter for ShellHelper {}
impl Validator for ShellHelper {}
impl Helper for ShellHelper {}
