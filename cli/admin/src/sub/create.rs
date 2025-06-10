//! Create a new, empty repository at REPOS_PATH.
//!

use svn_fs::FsFsConfig;

/// This function is part of the `svnadmin` CLI tool, which is used for
/// administrative tasks on Subversion repositories. The `create` function
pub fn create(repos_path: &str) -> anyhow::Result<()> {
    // TODO:
    let fs_config = FsFsConfig::default();

    svn_repos::Repos::create(repos_path, &fs_config)?;

    Ok(())
}
