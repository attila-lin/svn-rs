//! `svnadmin_opt_state`

use svn_subr::Version;
pub struct AdminContext {
    repository_path: String,
    fs_type: String,
    compatible_version: Version,
}
