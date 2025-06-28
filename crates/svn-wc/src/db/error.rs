#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum DBError {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
    #[error("Working copy database {0} not found")]
    NotFound(String),
}
