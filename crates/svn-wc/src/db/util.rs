//! `wc_db_util.c`

use super::DBError;
use super::SqliteMode;

pub fn open_db(smode: SqliteMode) -> Result<(), DBError> {
    let sdb_abspath = Adm::child(dir_abspath, sdb_fname);

    if smode != SqliteMode::RwCreate {
        let kind = svn_subr::io::check_path(&sdb_abspath)?;
        if kind != NodeKind::File {
            return Err(DBError::NotFound(sdb_abspath);
        }
    }

    Ok(())
}
