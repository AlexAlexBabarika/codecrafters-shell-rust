use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::commands::builtin_command_names::get_builtin_command_names;
use crate::util::path_utilities::{get_current_directory_entries, get_path_executables};

struct CompletionNamesCache {
    path_env: String,
    names: Arc<Vec<String>>,
}

static COMPLETION_CACHE: Mutex<Option<CompletionNamesCache>> = Mutex::new(None);

pub fn completion_names_cached() -> Arc<Vec<String>> {
    let path_key = env::var("PATH").unwrap_or_default();

    {
        let guard = COMPLETION_CACHE.lock().unwrap();
        if let Some(cache) = guard.as_ref() {
            if cache.path_env == path_key {
                return Arc::clone(&cache.names);
            }
        }
    }

    let mut merged = get_builtin_command_names(true);
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

static CURRENT_DIR_ENTRIES_CACHE: Mutex<Option<CurrentDirFilesCache>> = Mutex::new(None);

pub fn current_dir_entries_cached() -> Arc<Vec<String>> {
    let current_dir = env::current_dir().unwrap_or_default();

    {
        let guard = CURRENT_DIR_ENTRIES_CACHE.lock().unwrap();
        if let Some(cache) = guard.as_ref() {
            if cache.current_dir == current_dir {
                return Arc::clone(&cache.names);
            }
        }
    }

    let names = Arc::new(get_current_directory_entries().unwrap_or_default());
    let mut guard = CURRENT_DIR_ENTRIES_CACHE.lock().unwrap();
    *guard = Some(CurrentDirFilesCache {
        current_dir: current_dir,
        names: Arc::clone(&names),
    });

    names
}
