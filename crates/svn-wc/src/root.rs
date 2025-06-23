//! wcroot

use std::path::{Path, PathBuf};

use crate::Error;
use crate::sqlite::SqliteDb;

/// Hold information about a WCROOT.
///
/// This structure is referenced by all per-directory handles underneath it.
///
/// `svn_wc__db_wcroot_t`
#[derive(Debug)]
pub struct WcRoot {
    /// Location of this wcroot in the filesystem.
    abspath: PathBuf,

    /// The SQLite database containing the metadata for everything in
    /// this wcroot.
    pub(crate) sdb: SqliteDb,
}

impl WcRoot {
    /// `is_wclocked`
    pub fn is_wclocked<P>(&self, dir_relpath: P) -> bool
    where
        P: AsRef<PathBuf>,
    {
        // Check if the directory is locked in the working copy.
        // This is a placeholder implementation; actual logic will depend on the database schema.
        let relpath = dir_relpath.as_ref();
        self.find_wclock(relpath).ok().is_some()
    }

    /// The body of svn_wc__db_wclock_find_root() and svn_wc__db_wclocked().
    /// `find_wclock`
    pub fn find_wclock(&self, dir_relpath: &Path) -> Result<Option<String>, Error> {
        let dir_path = Self::relpath_depth(dir_relpath);

        /* Check for locks on all directories that might be ancestors.
        As our new apis only use recursive locks the number of locks stored
        in the DB will be very low */
        let res = self.ancestor_wclocks()?;

        todo!()
    }
}

/// utilities for WcRoot
impl WcRoot {
    /// Calculates the depth of the relpath below ""
    #[inline]
    pub fn relpath_depth<P>(relpath: P) -> u8
    where
        P: AsRef<Path>,
    {
        let p = relpath.as_ref();
        if p == Path::new("") {
            0
        } else {
            p.components().count() as u8
        }
    }
}
