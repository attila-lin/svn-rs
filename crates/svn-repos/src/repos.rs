use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use fs_err::File;
use svn_fs::FsConfig;
use svn_fs::FsFsConfig;
use svn_fs::FsType;
use svn_fs::SvnFs;
use svn_fs::backend::PATH_FORMAT;
use svn_ra::RaCapability;
use svn_types::NodeKind;
use svn_types::RevisionNumber;

use crate::Error;

// Repository format number.
//
//    Formats 0, 1 and 2 were pre-1.0.
//
//    Format 3 was current for 1.0 through to 1.3.
//
//    Format 4 was an abortive experiment during the development of the
//    locking feature in the lead up to 1.2.
//
//    Format 5 was new in 1.4, and is the first format which may contain
//    BDB or FSFS filesystems with a FS format other than 1, since prior
//    formats are accepted by some versions of Subversion which do not
//    pay attention to the FS format number.
const FORMAT_NUMBER: i32 = 5;
const FORMAT_NUMBER_LEGACY: i32 = 3;

// Repository layout.

/// The top-level repository dir contains a README and various subdirectories.
const README: &str = "README.txt";
/// Stores the current version of the repository format.
const FORMAT: &str = "format";
/// Where Berkeley lives
const DB_DIR: &str = "db";
/// DAV sandbox
const DAV_DIR: &str = "dav";
/// Lock files live here
const LOCKS_DIR: &str = "locks";
/// Hook programs
const HOOKS_DIR: &str = "hooks";
/// Configuration files
const CONF_DIR: &str = "conf";

/// The Repository object, created by `svn_repos_open2`() and
///   `svn_repos_create`().
#[derive(Default)]
pub struct Repos {
    /// A Subversion filesystem object.
    fs: Option<SvnFs>,
    /// The path to the repository's top-level directory.
    path: PathBuf,
    /// The path to the repository's conf directory.
    conf_path: PathBuf,

    /// The path to the repository's hooks directory.
    hooks_path: PathBuf,

    /// The path to the repository's hooks directory.
    hook_path: PathBuf,

    /// The path to the repository's locks directory.
    locks_path: PathBuf,

    /// The path to the Berkeley DB filesystem environment.
    db_path: PathBuf,

    /// The format number of this repository.
    format: i32,

    /// The path to the repository's hooks environment file. If NULL, hooks run
    //  / in an empty environment.
    hooks_env_path: Option<String>,

    /// The FS backend in use within this repository.
    fs_type: FsType,

    /// If non-null, a list of all the capabilities the client (on the
    /// current connection) has self-reported.  Each element is a
    /// 'const char *', one of SVN_RA_CAPABILITY_*.
    ///
    /// @note: it is somewhat counterintuitive that we store the client's
    /// capabilities, which are session-specific, on the repository
    /// object.  You'd think the capabilities here would represent the
    /// *repository's* capabilities, but no, they represent the
    /// client's -- we just don't have any other place to persist them.
    client_capabilities: Option<Vec<RaCapability>>,

    /// Maps SVN_REPOS_CAPABILITY_foo keys to "yes" or "no" values.
    //      If a capability is not yet discovered, it is absent from the table.
    //      Most likely the keys and values are constants anyway (and
    //      sufficiently well-informed internal code may just compare against
    //      those constants' addresses, therefore).
    repository_capabilities: HashMap<String, Vec<String>>,
}

impl Repos {
    /// Opens a repository at the given path.
    ///
    /// `svn_repos_open3`
    pub fn open(repository_path: &str) -> Result<Self, Error> {
        // Here we would normally open the repository and return a Repos instance.
        // For now, we just return an empty Repos instance.
        // Repos {
        //     // fs: Rc::new(Fs::new(repository_path)),
        // };
        let mut default_repo = Self::default();

        // Verify the validity of our repository format.
        default_repo.check_repos_format(repository_path)?;

        // Discover the FS type.
        // default_repo.

        todo!()
    }

    /// Verify that REPOS's format is suitable.
    ///
    /// `check_repos_format`
    fn check_repos_format(&mut self, repository_path: &str) -> Result<(), Error> {
        let format_path = Path::new(repository_path).join(PATH_FORMAT);
        let format_str = fs_err::read_to_string(&format_path)?;
        let format: i32 = format_str.parse().map_err(|_| Error::InvalidFormatFile {
            path: format_path.to_string_lossy().to_string(),
            error: "Failed to parse format".to_string(),
        })?;
        if format != FORMAT_NUMBER && format != FORMAT_NUMBER_LEGACY {
            return Err(Error::InvalidFormat {
                found: format,
                expected: FORMAT_NUMBER,
                expected2: FORMAT_NUMBER_LEGACY,
            });
        }
        self.format = format;

        Ok(())
    }

    /// Set @a *repos_p to a repository object for the repository at @a path.
    ///
    /// Allocate @a *repos_p in @a result_pool.
    ///
    /// Acquires a shared lock on the repository, and attaches a cleanup
    /// function to @a result_pool to remove the lock.  If no lock can be acquired,
    /// returns error, with undefined effect on @a *repos_p.  If an exclusive
    /// lock is present, this blocks until it's gone.  @a fs_config will be
    /// passed to the filesystem initialization function and may be @c NULL.
    ///
    /// Use @a scratch_pool for temporary allocations.
    pub fn open_path(&self, path: &Path, fs_config: &FsConfig) -> Result<Repos, Error> {
        todo!()
    }

    /// `svn_repos_create`
    ///
    /// FIXME: config is not used yet.
    pub fn create(
        repository_path: &str,
        config: &Option<HashMap<String, String>>,
        fs_config: &FsConfig,
    ) -> Result<Self, Error> {
        let mut repos = Self::default();
        repos._create(repository_path, fs_config)?;
        repos.format = 5; // read `SVN_REPOS__FORMAT_NUMBER`

        // Discover the type of the filesystem we are about to create.
        repos.fs_type = fs_config.fs_type();

        // Don't create a repository inside another repository.
        let local_abspath = todo!();

        Ok(repos)
    }

    /// Creates a new repository at the given path.
    ///
    /// **Note:** This is a private method and should not be called directly.
    ///
    /// `svn_repos_create`
    /// `create_repos_structure`
    fn _create(&mut self, repository_path: &str, fs_config: &FsConfig) -> Result<(), Error> {
        // `create_svn_repos_t`
        self.path = PathBuf::from(repository_path);
        self.db_path = self.path.join(DB_DIR);
        self.conf_path = self.path.join(DB_DIR);
        self.hooks_path = self.path.join(HOOKS_DIR);
        self.locks_path = self.path.join(LOCKS_DIR);

        self.format = FORMAT_NUMBER;

        // Discover the type of the filesystem we are about to create.
        self.fs_type = fs_config.fs_type();

        // Don't create a repository inside another repository
        if let Some(root_path) = Self::find_root_path(&self.path) {
            if root_path == self.path {
                return Err(Error::IsExistingRepository(root_path));
            } else {
                return Err(Error::IsSubRepositoryOfExistingRepository {
                    root_path,
                    sub_path: self.path.clone(),
                });
            }
        }

        // Create the various files and subdirectories for the repository
        self.create_repos_structure(&self.path, fs_config)?;

        // Lock if needed.
        #[allow(deprecated)]
        self.lock(false, false)?;

        // // Create an environment for the filesystem.
        // if let Err(e) = SvnFs::create(&mut self.fs, &self.db_path, fs_config) {
        //     // If there was an error making the filesystem, e.g. unknown/supported
        //     // filesystem type.  Clean up after ourselves.  Yes this is safe because
        //     // create_repos_structure will fail if the path existed before we started
        //     // so we can't accidentally remove a directory that previously existed.
        //     if self.path.exists() {
        //         fs_err::remove_dir_all(&self.path)?;
        //     }
        //     return Err(e);
        // }

        todo!()
    }

    /// `create_repos_structure`
    fn create_repos_structure(&self, path: &Path, fs_config: &FsConfig) -> Result<(), Error> {
        // Create the top-level repository directory.
        if !path.exists() {
            fs_err::create_dir_all(path)?;
        }

        // Create the DAV sandbox directory if pre-1.4 or pre-1.5-compatible.
        // FIXME: don't care DAV sandbox for now.

        // Create the lock directory.
        fs_err::create_dir_all(&self.locks_path)?;

        // Create the hooks directory.
        fs_err::create_dir_all(&self.hooks_path)?;

        // Create the conf directory.
        fs_err::create_dir_all(&self.conf_path)?;

        // Write the top-level README file.
        {
            let readme_header = r#"
            This is a Subversion repository; use the 'svnadmin' and 'svnlook'
            tools to examine it.  Do not add, delete, or modify files here
            unless you know how to avoid corrupting the repository.
            "#;
            let readme_bdb_insert = format!(
                r#"
            The directory {DB_DIR} contains a Berkeley DB environment.
            you may need to tweak the values in {DB_DIR}/DB_CONFIG" to match the
            requirements of your site.
            "#
            );
            let readme_footer = "Visit https://subversion.apache.org/ for more information.";

            let mut readme_file = File::create(path.join(README))?;
            readme_file.write_all(readme_header.as_bytes())?;
            #[allow(deprecated)]
            if self.fs_type == FsType::Bdb {
                readme_file.write_all(readme_bdb_insert.as_bytes())?;
            }
            readme_file.write_all(readme_footer.as_bytes())?;
            readme_file.flush()?;
        }

        Ok(())
    }

    /// Find the root path of the repository that contains @a path.
    ///
    /// If a repository was found, the path to the root of the repository
    /// is returned, else @c NULL. The pointer to the returned path may be
    /// equal to @a path.
    ///
    /// `svn_repos_find_root_path`
    fn find_root_path(path: &Path) -> Option<PathBuf> {
        let mut current = path;
        while let Some(parent) = current.parent() {
            if Self::check_repos_path(parent) {
                return Some(parent.to_path_buf());
            }
            current = parent;
        }
        None
    }

    /// Check if @a path is the root of a repository by checking if the
    /// path contains the expected files and directories.  Return TRUE
    /// on errors (which would be permission errors, probably) so that
    /// we the user will see them after we try to open the repository
    /// for real.
    ///
    /// `check_repos_path`
    fn check_repos_path(path: &Path) -> bool {
        let kind = match Self::check_path(&path.join(FORMAT)) {
            Err(_) => return true,
            Ok(kind) => kind,
        };

        if kind != NodeKind::File {
            return false;
        }

        // Check the db/ subdir, but allow it to be a symlink (Subversion works just fine if it's a symlink).
        match Self::check_resolved_path(&path.join(DB_DIR)) {
            Err(_) => return true,
            Ok(kind) => {
                if kind != NodeKind::Directory && kind != NodeKind::Symlink {
                    return false;
                }
            }
        }

        true
    }

    ////** Determine the @a kind of @a path.  @a path should be UTF-8 encoded.
    ///
    /// If @a path is a file, set @a *kind to #svn_node_file.
    ///
    /// If @a path is a directory, set @a *kind to #svn_node_dir.
    ///
    /// If @a path does not exist, set @a *kind to #svn_node_none.
    ///
    /// If @a path exists but is none of the above, set @a *kind to
    /// #svn_node_unknown.
    ///
    /// If @a path is not a valid pathname, set @a *kind to #svn_node_none.  If
    /// unable to determine @a path's kind for any other reason, return an error,
    /// with @a *kind's value undefined.
    ///
    /// Use @a pool for temporary allocations.
    ///
    /// @see svn_node_kind_t
    ///
    /// `svn_io_check_path`
    fn check_path(path: &Path) -> Result<NodeKind, Error> {
        // FIXME: win symlinks
        if !path.exists() {
            return Ok(NodeKind::None);
        }
        if path.is_file() {
            return Ok(NodeKind::File);
        }
        if path.is_dir() {
            return Ok(NodeKind::Directory);
        }
        Ok(NodeKind::Unknown)
    }

    /// Like svn_io_check_path(), but resolve symlinks.  This returns the
    /// same varieties of @a kind as svn_io_check_path()
    /// `svn_io_check_resolved_path`
    fn check_resolved_path(path: &Path) -> Result<NodeKind, Error> {
        // This function checks the resolved path, which means it follows symlinks.
        // For now, we just call `check_path` since it already handles the logic.
        Self::check_path(path)
    }

    /// There is, at present, nothing within the direct responsibility
    /// of libsvn_repos which requires locking.  For historical compatibility
    /// reasons, the BDB libsvn_fs backend does not do its own locking, expecting
    /// libsvn_repos to do the locking for it.  Here we take care of that
    /// backend-specific requirement.
    /// The kind of lock is controlled by EXCLUSIVE and NONBLOCKING.
    ///
    /// `lock_repos`
    #[deprecated = "only use with BDB repositories"]
    fn lock(&self, _exclusive: bool, _nonblocking: bool) -> Result<(), Error> {
        Ok(())
    }
}

/// get
impl Repos {
    /// Get the youngest revision number.
    pub fn youngest_rev(&self) -> Result<RevisionNumber, Error> {
        // Here we would normally retrieve the youngest revision number from the repository.
        // For now, we just return a dummy value.
        Ok(0)
    }
}
