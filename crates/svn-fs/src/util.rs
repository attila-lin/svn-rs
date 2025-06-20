//! fs-util.c : internal utility functions used by both FSFS and BDB back
//! ends.

use std::collections::HashMap;

/// `svn_fs__prop_lists_equal`
pub fn prop_lists_equal(a: &HashMap<String, String>, b: &HashMap<String, String>) -> bool {
    if a == b {
        return true;
    }

    if a.len() != b.len() {
        return false;
    }

    for (k, v) in a {
        if let Some(other_v) = b.get(k) {
            if v != other_v {
                return false;
            }
        } else {
            return false;
        }
    }
    for (k, v) in b {
        if let Some(other_v) = a.get(k) {
            if v != other_v {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}
