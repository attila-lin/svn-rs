use crate::{AccessType, UsernameCaseType};
use svn_fs::SvnFs;
use svn_ra::RaCapability;
use svn_repos::Repos;
use svn_repos::SvnAuthz;
use svn_subr::SvnConfig;
use uuid::Uuid;

/// `repository_t`
pub struct Repository {
    repos: Repos,

    /// URI-encoded name of repository (not for authz)
    repos_name: String,
    /// Repository root directory
    repos_root: String,

    /// For convenience; same as svn_repos_fs(repos)
    fs: SvnFs,

    /// Base directory for config files
    base: String,
    /// Parsed password database
    pw_db: SvnConfig,
    /// Parsed authz rules
    authz_db: SvnAuthz,

    /// The name of the repository for authz
    authz_repos_name: String,
    /// Authentication realm
    realm: String,
    /// URL to base of repository
    repos_url: String,
    /// Path to the hooks environment file or NULL
    hooks_env_path: Option<String>,
    /// Repository ID
    uuid: Uuid,

    /// Client capabilities (SVN_RA_CAPABILITY_*)
    capabilities: Vec<RaCapability>,

    /// Decoded base in-repos path (w/ leading slash)
    fs_path: String,

    /// Case-normalize the username?
    username_case: UsernameCaseType,

    /// Use Cyrus SASL for authentication;
    /// always false if `SVN_HAVE_SASL` not defined
    use_sasl: bool,

    /// min-encryption SASL parameter
    #[cfg(features = "_sasl")]
    min_ssf: u32,
    /// max-encryption SASL parameter
    #[cfg(features = "_sasl")]
    max_ssf: u32,

    /// access granted to authenticated users
    pub auth_access: AccessType,
    /// access granted to anonymous users
    anonymous_access: AccessType,
}
