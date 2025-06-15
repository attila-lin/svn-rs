#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),

    #[error("File exists: {0}")]
    FileExists(String),

    #[error("File not found: {0}")]
    FileNotFound(String),
}
