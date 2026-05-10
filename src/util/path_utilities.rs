use std::collections::HashSet;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub fn check_if_executable(path: &str) -> bool {
    is_executable_file(Path::new(path))
}

fn is_executable_file(path: &Path) -> bool {
    let Ok(meta) = fs::metadata(path) else {
        return false;
    };
    if meta.is_dir() {
        return false;
    }
    meta.permissions().mode() & 0o100 != 0
}

pub fn find_executable(exe_name: &str) -> Option<String> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths).find_map(|dir| {
        let full_path = dir.join(exe_name);
        full_path
            .to_str()
            .filter(|p| check_if_executable(p))
            .map(str::to_string)
    })
}

pub fn get_path_executables() -> Result<Vec<String>, env::VarError> {
    let path_var = env::var("PATH")?;
    let mut seen = HashSet::new();

    for dir in env::split_paths(&path_var) {
        let Ok(read_dir) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in read_dir.filter_map(Result::ok) {
            if !is_executable_file(&entry.path()) {
                continue;
            }
            if let Some(name) = entry.file_name().to_str() {
                seen.insert(name.to_string());
            }
        }
    }

    let mut names: Vec<String> = seen.into_iter().collect();
    names.sort();
    Ok(names)
}

pub fn get_current_directory_files() -> Result<Vec<String>, std::io::Error> {
    let current_dir = env::current_dir()?;
    let mut files = Vec::new();

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            files.push(name.to_string());
        }
    }

    Ok(files)
}

pub fn get_matches_in_directory(
    dir: &str,
    file_prefix: &str,
) -> Result<std::sync::Arc<Vec<String>>, std::io::Error> {
    let dir_path = if dir.starts_with('/') {
        PathBuf::from(dir)
    } else {
        env::current_dir()?.join(dir)
    };

    let include_hidden = file_prefix.starts_with('.');
    let mut files = Vec::new();
    if let Ok(rd) = fs::read_dir(&dir_path) {
        for entry in rd.filter_map(Result::ok) {
            let Some(name) = entry.file_name().to_str().map(str::to_string) else {
                continue;
            };
            if !name.starts_with(file_prefix) {
                continue;
            }
            if !include_hidden && name.starts_with('.') {
                continue;
            }
            files.push(format!("{}{}", dir, name));
        }
    }
    Ok(std::sync::Arc::new(files))
}
