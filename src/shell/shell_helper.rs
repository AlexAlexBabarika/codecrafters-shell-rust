use std::env;
use std::sync::{Arc, Mutex};

use crate::commands::builtin_command_names::get_builtin_command_names;
use crate::util::path_utilities::{get_current_directory_files, get_path_executables};

use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};
use std::path::PathBuf;

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

struct CurrentDirFilesCache {
    current_dir: PathBuf,
    names: Arc<Vec<String>>,
}

static CURRENT_DIR_FILES_CACHE: Mutex<Option<CurrentDirFilesCache>> = Mutex::new(None);

fn current_dir_files_cached() -> Arc<Vec<String>> {
    let current_dir = env::current_dir().unwrap_or_default();

    {
        let guard = CURRENT_DIR_FILES_CACHE.lock().unwrap();
        if let Some(cache) = guard.as_ref() {
            if cache.current_dir == current_dir {
                return Arc::clone(&cache.names);
            }
        }
    }

    let names = Arc::new(get_current_directory_files().unwrap_or_default());
    let mut guard = CURRENT_DIR_FILES_CACHE.lock().unwrap();
    *guard = Some(CurrentDirFilesCache {
        current_dir: current_dir,
        names: Arc::clone(&names),
    });

    names
}

pub struct ShellHelper;

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let start = line[..pos]
            .rfind(char::is_whitespace)
            .map(|i| i + 1)
            .unwrap_or(0);
        let prefix = &line[start..pos];

        let is_first_word = line[..start].trim().is_empty();
        let names = if is_first_word {
            completion_names_cached()
        } else {
            current_dir_files_cached()
        };

        let include_hidden = prefix.starts_with('.');
        let matches: Vec<Pair> = names
            .iter()
            .filter(|b| b.starts_with(prefix))
            .filter(|b| include_hidden || !b.starts_with('.'))
            .map(|b| Pair {
                display: b.clone(),
                replacement: format!("{} ", b),
            })
            .collect();

        Ok((start, matches))
    }
}

impl Hinter for ShellHelper {
    type Hint = String;
}
impl Highlighter for ShellHelper {}
impl Validator for ShellHelper {}
impl Helper for ShellHelper {}
