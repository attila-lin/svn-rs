#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum DBError {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
}
