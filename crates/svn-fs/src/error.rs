use crate::FsType;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UnsupportedFsType: {0:?}")]
    UnsupportedFsType(FsType),

    #[error(transparent)]
    Backend(#[from] crate::backend::BackendError),
}
