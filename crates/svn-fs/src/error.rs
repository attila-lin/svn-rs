#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("UnsupportedFsType: {0:?}")]
    UnsupportedFsType(crate::FsType),

    #[error(transparent)]
    Backend(#[from] crate::backend::BackendError),

    #[error(transparent)]
    Config(#[from] svn_subr::config::ConfigError),
}
