use std::collections::HashMap;

/// `svn_hash__get_bool`
pub fn get_bool(map: &HashMap<String, String>, key: &str, default_value: bool) -> bool {
    match map.get(key) {
        Some(value) => value.parse().unwrap_or(default_value),
        None => default_value,
    }
}
