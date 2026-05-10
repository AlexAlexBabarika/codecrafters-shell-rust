use std::env;
use std::sync::{Arc, Mutex};

use crate::commands::builtin_command_names::get_builtin_command_names;
use crate::util::find_executable::get_path_executables;

use rustyline::completion::{Completer, Pair};
// use rustyline::error::FilenameCompleter;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};

struct CompletionNamesCache {
    path_env: String,
    names: Arc<Vec<String>>,
}

static COMPLETION_CACHE: Mutex<Option<CompletionNamesCache>> = Mutex::new(None);

fn completion_names_cached() -> Arc<Vec<String>> {
    let path_key = env::var("PATH").unwrap_or_default();

    {
        let guard = COMPLETION_CACHE.lock().unwrap();
        if let Some(cache) = guard.as_ref() {
            if cache.path_env == path_key {
                return Arc::clone(&cache.names);
            }
        }
    }

    let mut merged = get_builtin_command_names();
    merged.extend(get_path_executables().unwrap_or_default());
    merged.sort();
    merged.dedup();
    let names = Arc::new(merged);

    let mut guard = COMPLETION_CACHE.lock().unwrap();
    *guard = Some(CompletionNamesCache {
        path_env: path_key,
        names: Arc::clone(&names),
    });

    names
}

pub struct ShellHelper;

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let prefix = &line[..pos];
        let candidates = completion_names_cached();

        let matches: Vec<Pair> = candidates
            .iter()
            .filter(|b| b.starts_with(prefix))
            .map(|b| Pair {
                display: b.clone(),
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
