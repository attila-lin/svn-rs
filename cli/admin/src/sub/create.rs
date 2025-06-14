//! Create a new, empty repository at REPOS_PATH.
//!

use svn_fs::FsConfig;
use svn_repos::Repos;

/// This function is part of the `svnadmin` CLI tool, which is used for
/// administrative tasks on Subversion repositories. The `create` function
///
/// `subcommand_create`
pub fn create(repos_path: &str) -> anyhow::Result<()> {
    let fs_config = FsConfig::default();

    Repos::create(repos_path, &None, &fs_config)?;

    // TODO:

    Ok(())
}
