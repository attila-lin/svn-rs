//! wcroot

use std::path::PathBuf;

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
    sdb: SqliteDb,
}
