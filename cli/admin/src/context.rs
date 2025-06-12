//! `svnadmin_opt_state`

use svn_subr::Version;

/// `svnadmin_opt_state`
pub struct AdminContext {
    repository_path: String,
    fs_type: String,
    compatible_version: Version,
}
