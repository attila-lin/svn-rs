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

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Config(#[from] svn_subr::config::ConfigError),

    #[error(transparent)]
    Caching(#[from] crate::backend::fsfs::CachingError),
}
