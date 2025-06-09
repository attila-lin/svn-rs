use svn_fs::SvnFs;
use svn_repos::Repos;

pub struct Repository {
    repos: Repos,

    /// URI-encoded name of repository (not for authz)
    repos_name: String,
    /// Repository root directory
    repos_root: String,

    /// For convenience; same as svn_repos_fs(repos)
    fs: SvnFs,
}
