use std::path::PathBuf;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum DBError {
    #[error(transparent)]
    Db(#[from] rusqlite::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Sqlite(#[from] crate::sqlite::SqliteError),
    #[error("Working copy database {0:?} not found")]
    NotFound(PathBuf),
}
