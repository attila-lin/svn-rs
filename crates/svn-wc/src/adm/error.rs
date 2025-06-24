#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum AdmError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("{0} is not a valid administrative direcotry name")]
    InvalidAdmDirName(String),
}
