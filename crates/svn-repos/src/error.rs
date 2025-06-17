use std::path::PathBuf;

use thiserror::Error;

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Invalid format file: path={path}, error={error}")]
    InvalidFormatFile { path: String, error: String },

    #[error("Invalid format: found {found}, expected {expected} and {expected2}")]
    InvalidFormat {
        found: i32,
        expected: i32,
        expected2: i32,
    },

    #[error("{0} is an existing repository")]
    IsExistingRepository(PathBuf),

    #[error("{sub_path} is a subdirectory of an existing repository at {root_path}")]
    IsSubRepositoryOfExistingRepository {
        root_path: PathBuf,
        sub_path: PathBuf,
    },
}
