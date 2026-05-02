use std::path;
use std::fs;
use std::env;
use std::os::unix::fs::PermissionsExt;

fn check_if_executable(exe_dir: &str) -> bool {
    let path = path::Path::new(exe_dir);
    if path.is_dir() { return false; }

    if let Ok(metadata) = fs::metadata(path) {
        let permissions = metadata.permissions();
        if permissions.mode() & 0o100 != 0 {
            return true;
        }
    }
    false 
}

pub fn find_executable(exe_name: &str) -> Option<String> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(exe_name);
            if let Some(string_path) = full_path.to_str() {
                if check_if_executable(string_path) {
                    Some(string_path.to_string())
                } else {
                    None
                }
            }
            else {
                None
            }
        }).next()
    })    
}