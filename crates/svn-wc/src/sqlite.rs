/// `svn_sqlite__db_t`
#[derive(Debug)]
pub struct SqliteDb {
    pub conn: rusqlite::Connection,
    nbr_statements: usize,
}
