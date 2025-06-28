#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Not absolute path: {0}")]
    NotAbsolutePath(String),

    #[error(transparent)]
    Adm(#[from] crate::adm::AdmError),

    #[error(transparent)]
    Db(#[from] crate::db::DBError),

    #[error(transparent)]
    Config(#[from] svn_subr::config::ConfigError),
}
