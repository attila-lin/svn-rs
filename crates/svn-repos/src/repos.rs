use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use svn_fs::FsConfig;
use svn_fs::FsFsConfig;
use svn_fs::FsType;

use crate::Error;

/// The Repository object, created by svn_repos_open2() and
//    svn_repos_create().
pub struct Repos {
    /// A Subversion filesystem object.
    fs: Rc<dyn svn_fs::FsTrait>,
    /// The path to the repository's top-level directory.
    path: String,
    /// The path to the repository's conf directory.
    conf_path: String,

    /// The path to the repository's hooks directory.
    hooks_path: String,

    /// The path to the repository's hooks directory.
    hook_path: String,

    /// The path to the repository's locks directory.
    locks_path: String,

    /// The path to the Berkeley DB filesystem environment.
    db_path: String,

    /// The format number of this repository.
    format: i32,

    /// The path to the repository's hooks environment file. If NULL, hooks run
    //    * in an empty environment.
    hooks_env_path: Option<String>,

    /// The FS backend in use within this repository.
    fs_type: FsType,

    /// If non-null, a list of all the capabilities the client (on the
    //      current connection) has self-reported.  Each element is a
    //      'const char *', one of SVN_RA_CAPABILITY_*.
    //
    //      @note: it is somewhat counterintuitive that we store the client's
    //      capabilities, which are session-specific, on the repository
    //      object.  You'd think the capabilities here would represent the
    //      *repository's* capabilities, but no, they represent the
    //      client's -- we just don't have any other place to persist them.
    client_capabilities: Option<Vec<String>>,

    /// Maps SVN_REPOS_CAPABILITY_foo keys to "yes" or "no" values.
    //      If a capability is not yet discovered, it is absent from the table.
    //      Most likely the keys and values are constants anyway (and
    //      sufficiently well-informed internal code may just compare against
    //      those constants' addresses, therefore).
    repository_capabilities: HashMap<String, Vec<String>>,
}

impl Repos {
    /// Opens a repository at the given path.
    pub fn open(repository_path: &str) -> Result<Self, Error> {
        // Here we would normally open the repository and return a Repos instance.
        // For now, we just return an empty Repos instance.
        // Repos {
        //     // fs: Rc::new(Fs::new(repository_path)),
        // };

        todo!()
    }

    /// Set @a *repos_p to a repository object for the repository at @a path.
    //  *
    //  * Allocate @a *repos_p in @a result_pool.
    //  *
    //  * Acquires a shared lock on the repository, and attaches a cleanup
    //  * function to @a result_pool to remove the lock.  If no lock can be acquired,
    //  * returns error, with undefined effect on @a *repos_p.  If an exclusive
    //  * lock is present, this blocks until it's gone.  @a fs_config will be
    //  * passed to the filesystem initialization function and may be @c NULL.
    //  *
    //  * Use @a scratch_pool for temporary allocations.
    pub fn open_path(&self, path: &Path, fs_config: &FsConfig) -> Result<Repos, Error> {
        todo!()
    }

    /// Creates a new repository at the given path.
    ///
    /// `svn_repos_create`
    /// `create_repos_structure`
    fn create(repository_path: &str, fs_fs_config: &FsConfig) -> Self {
        // Here we would normally create a new repository and return a Repos instance.
        // For now, we just return an empty Repos instance.
        // Repos {
        //     // fs: Rc::new(Fs::new(repository_path)),
        // }

        let fs_type = fs_fs_config.fs_type();

        todo!()
    }
}
