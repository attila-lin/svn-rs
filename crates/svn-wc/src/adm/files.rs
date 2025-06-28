//! Names and file/dir operations in the administrative area.

use std::path::Path;

use svn_subr::SvnConfig;
use svn_types::{Depth, RevisionNumber};
use url::Url;
use uuid::Uuid;

use crate::WcDb;
use crate::adm::ADM_PRISTINE;

use super::ADM_DIR_NAME;
use super::Adm;
use super::AdmError;
use super::DEFAULT_ADM_DIR_NAME;

impl Adm {
    /// `svn_wc_is_adm_dir`
    pub fn is_adm_dir(name: &str) -> bool {
        name == ADM_DIR_NAME || name == DEFAULT_ADM_DIR_NAME
    }

    /// `svn_wc_set_adm_dir`
    pub fn set_adm_dir(name: &str) -> Result<(), AdmError> {
        const VALID_DIR_NAMES: [&str; 2] = [DEFAULT_ADM_DIR_NAME, "_svn"];

        if !VALID_DIR_NAMES.contains(&name) {
            Ok(())
        } else {
            Err(AdmError::InvalidAdmDirName(name.to_string()))
        }
    }

    /// `make_adm_subdir`
    fn make_adm_subdir(path: &Path, subdir: &str) -> Result<(), AdmError> {
        let adm_path = path.join(subdir);
        if !adm_path.exists() {
            fs_err::create_dir_all(&adm_path)?;
        }
        Ok(())
    }
    /// Set up a new adm area for PATH, with REPOS_* as the repos info, and
    /// INITIAL_REV as the starting revision.  The entries file starts out
    /// marked as 'incomplete.  The adm area starts out locked; remember to
    /// unlock it when done.
    ///
    /// `init_adm`
    pub fn init(
        config: &SvnConfig,
        target_format: i32,
        local_abspath: &Path,
        repos_relpath: &Path,
        repos_root_url: &Url,
        repos_uuid: &Uuid,
        initial_rev: RevisionNumber,
        depth: Depth,
        store_pristine: bool,
    ) -> Result<WcDb, crate::Error> {
        /* First, make an empty administrative area. */
        let root = Path::new(local_abspath);
        if !root.exists() {
            fs_err::create_dir_all(root)?;
            svn_subr::io::dir_make_hidden(root)?;
        }

        // Make subdirectories

        /* SVN_WC__ADM_PRISTINE */
        Self::make_adm_subdir(local_abspath, ADM_PRISTINE)?;

        // Init the tmp area.
        Self::make_adm_subdir(local_abspath, super::ADM_TMP)?;

        // Create the SDB.
        let wc = WcDb::init(
            config,
            target_format,
            local_abspath,
            repos_relpath,
            repos_root_url,
            repos_uuid,
            initial_rev,
            depth,
            store_pristine,
        )?;

        Ok(wc)
    }
}
