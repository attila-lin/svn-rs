//! `create_conf(svn_repos_t *repos, apr_pool_t *pool)`

use std::path::{Path, PathBuf};

use svn_fs::FsRoot;

use crate::Repos;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {}

/// `config_access_t`
pub struct ConfigAccess {
    /// The last repository that we found the requested URL in.  May be NULL.
    repos: Option<Repos>,
}

/// A stream object that gives access to a representation's content but
/// delays accessing the repository data until the stream is first used.
/// IOW, the stream object is cheap as long as it is not accessed.
///
/// `repr_steam_baton_t`
struct ReprStreamBaton {
    root: FsRoot,
    fs_path: PathBuf,
}

pub struct Config {}

impl Config {
    /// `get_file_config`
    pub fn from_path(path: &Path) -> Result<Self, ConfigError> {
        todo!()
    }
}
