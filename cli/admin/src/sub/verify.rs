use svn_fs::FsFsConfig;
use svn_repos::Repos;

pub fn verify(repository_path: &str) -> anyhow::Result<()> {
    let repos = Repos::open(repository_path)?;

    let mut fs_config = FsFsConfig::default();

    Ok(())
}
