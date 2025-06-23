#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not absolute path: {0}")]
    NotAbsolutePath(String),

    #[error(transparent)]
    Db(#[from] crate::db::DBError),
}
