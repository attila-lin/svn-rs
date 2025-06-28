pub mod queries;

use std::{path::Path, time::Duration};

use rusqlite::Connection;

use crate::db::SqliteMode;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum SqliteError {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
}

/// `svn_sqlite__db_t`
#[derive(Debug)]
pub struct SqliteDb {
    pub conn: rusqlite::Connection,
    nbr_statements: usize,
}

impl SqliteDb {
    /// Open a connection in *DB to the database at PATH. Validate the schema,
    /// creating/upgrading to LATEST_SCHEMA if needed using the instructions
    /// in UPGRADE_SQL. The resulting DB is allocated in RESULT_POOL, and any
    /// temporary allocations are made in SCRATCH_POOL.
    ///
    /// STATEMENTS is an array of strings which may eventually be executed, the
    /// last element of which should be NULL.  These strings and the array itself
    /// are not duplicated internally, and should have a lifetime at least as long
    /// as RESULT_POOL.
    /// STATEMENTS itself may be NULL, in which case it has no impact.
    /// See svn_sqlite__get_statement() for how these strings are used.
    ///
    /// TIMEOUT defines the SQLite busy timeout, values <= 0 cause a Subversion
    /// default to be used.
    ///
    /// The statements will be finalized and the SQLite database will be closed
    /// when RESULT_POOL is cleaned up.
    ///
    /// `svn_sqlite__open`
    pub fn open(
        path: &Path,
        mode: SqliteMode,
        statements: &[String],
        _lastest_schema: i32,
        _upgrade_sql: Option<&str>,
        timeout: i32,
    ) -> Result<Self, SqliteError> {
        let conn = Self::_open(path, mode, timeout)?;

        todo!()
    }

    /// `internal_open`
    fn _open(path: &Path, mode: SqliteMode, timeout: i32) -> Result<Connection, SqliteError> {
        let flags = match mode {
            SqliteMode::ReadOnly => rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
            SqliteMode::ReadWrite => rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
            SqliteMode::RwCreate => {
                rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE
                    | rusqlite::OpenFlags::SQLITE_OPEN_CREATE
            }
        };
        let conn = Connection::open_with_flags(path, flags)?;
        let timeout = if timeout <= 0 {
            const BUSY_TIMEOUT: i32 = 10000;
            BUSY_TIMEOUT
        } else {
            timeout
        };
        conn.busy_timeout(Duration::from_secs(timeout as u64))?;
        Ok(conn)
    }
    
    /// `svn_sqlite__exec_statements`
    pub fn exec_statements(& self, stmt: &str) -> Result<(), SqliteError> {
        let  conn = &self.conn;
        conn.execute(stmt, [])?;
        Ok(())
    }
}
