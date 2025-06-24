mod error;
pub use error::AdmError;

use std::path::Path;

use svn_types::{Depth, RevisionNumber};
use uuid::Uuid;

use crate::WcDb;

/// The default name of the WC admin directory. This name is always
/// checked by svn_wc_is_adm_dir.
const DEFAULT_ADM_DIR_NAME: &str = ".svn";

/// The name that is actually used for the WC admin directory.  The
/// commonest case where this won't be the default is in Windows
/// ASP.NET development environments, which used to choke on ".svn".
const ADM_DIR_NAME: &str = DEFAULT_ADM_DIR_NAME;

pub struct Adm;

impl Adm {
    /// Set up a new adm area for PATH, with REPOS_* as the repos info, and
    /// INITIAL_REV as the starting revision.  The entries file starts out
    /// marked as 'incomplete.  The adm area starts out locked; remember to
    /// unlock it when done.
    ///
    /// `init_adm`
    pub fn init(
        db: &WcDb,
        target_format: i32,
        local_abspath: &str,
        repos_relpath: &str,
        repso_root_url: &str,
        repos: &Uuid,
        initial_rev: RevisionNumber,
        depth: Depth,
        store_pristine: bool,
    ) -> Result<(), AdmError> {
        let root = Path::new(local_abspath);
        if !root.exists() {
            fs_err::create_dir_all(root)?;
            svn_subr::io::dir_make_hidden(root)?;
        }

        Ok(())
    }
}
