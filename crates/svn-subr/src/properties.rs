use std::collections::HashMap;

pub fn has_svn_prop(props: &HashMap<String, String>) -> bool {
    if props.is_empty() {
        return false;
    }

    for (key, _) in props.iter() {
        if is_svn_prop(key) {
            return true;
        }
    }

    false
}

#[inline]
pub fn is_svn_prop(prop_name: &str) -> bool {
    prop_name.starts_with("svn:")
}
