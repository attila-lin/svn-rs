//! `wc_db_util.c`

use std::path::Path;

use svn_types::NodeKind;

use super::DBError;
use super::SqliteMode;
use crate::Adm;
use crate::sqlite::SqliteDb;

/// `svn_wc__db_util_open_db`
pub fn open_db(
    dir_abspath: &Path,
    sdb_fname: &str,
    smode: SqliteMode,
    exclusive: bool,
    timeout: i32,
    my_statements: &[String],
) -> Result<(), DBError> {
    let sdb_abspath = Adm::child(dir_abspath, sdb_fname);

    if smode != SqliteMode::RwCreate {
        let kind = svn_subr::io::check_path(&sdb_abspath)?;
        if kind != NodeKind::File {
            return Err(DBError::NotFound(sdb_abspath));
        }
    }

    let sdb = SqliteDb::open(sdb_abspath, smode, my_statements, timeout);

    Ok(())
}
