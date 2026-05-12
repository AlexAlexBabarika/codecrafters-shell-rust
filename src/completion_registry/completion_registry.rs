use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

fn completions_map() -> &'static Mutex<HashMap<String, String>> {
    static MAP: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn register_completion(command_name: &String, completion_spec: &String) {
    completions_map()
        .lock()
        .unwrap()
        .insert(command_name.to_string(), completion_spec.to_string());
}

pub fn get_completion(command_name: &String) -> Option<String> {
    completions_map().lock().unwrap().get(command_name).cloned()
}
