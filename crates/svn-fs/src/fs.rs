use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use uuid::Uuid;

use crate::Error;
use crate::FsType;
use crate::backend::fsfs::{FsFsBackend, FsFsData};
use crate::backend::{FsInstance, FsLibrary};
use crate::{FsAccess, FsConfig};

const FS_TYPE_FILENAME: &str = "fs-type";

/// An object representing a Subversion filesystem.
///
/// `svn_fs_t`
pub struct SvnFs {
    ///  The path to the repository's top-level directory
    pub path: PathBuf,

    /// The filesystem configuration
    pub config: HashMap<String, String>,

    /// An access context indicating who's using the fs
    pub access_ctx: FsAccess,

    // **Note:** Moved to [`inner`]
    // /// FSAP-specific vtable and private data
    // ///
    // /// FIXME: now use [`FsFsData`]
    // data: Option<FsFsData>,
    /// The filesystem library that implements this filesystem
    inner: Option<Box<dyn FsInstance>>,

    /// UUID, stored by open(), create(), and set_uuid().
    pub uuid: Uuid,
}

/// utilities
impl SvnFs {
    pub fn inner(&self) -> &Box<dyn FsInstance> {
        self.inner.as_ref().expect("SvnFs inner is not set")
    }
    pub fn inner_mut(&mut self) -> &mut Box<dyn FsInstance> {
        self.inner.as_mut().expect("SvnFs inner is not set")
    }
}

impl SvnFs {
    /// Create a new, empty Subversion filesystem, stored in the directory
    /// @a path, and return a pointer to it in @a *fs_p.  @a path must not
    /// currently exist, but its parent must exist.  If @a fs_config is not
    /// @c NULL, the options it contains modify the behavior of the
    /// filesystem.  The interpretation of @a fs_config is specific to the
    /// filesystem back-end.  The new filesystem may be closed by
    /// destroying @a result_pool.
    ///
    /// Use @a scratch_pool for temporary allocations.
    ///
    /// @note The lifetime of @a fs_config must not be shorter than @a
    /// result_pool's. It's a good idea to allocate @a fs_config from
    /// @a result_pool or one of its ancestors.
    ///
    /// If @a fs_config contains a value for #SVN_FS_CONFIG_FS_TYPE, that
    /// value determines the filesystem type for the new filesystem.
    /// Currently defined values are:
    ///
    ///   SVN_FS_TYPE_BDB   Berkeley-DB implementation
    ///   SVN_FS_TYPE_FSFS  Native-filesystem implementation
    ///   SVN_FS_TYPE_FSX   Experimental filesystem implementation
    ///
    /// If @a fs_config is @c NULL or does not contain a value for
    /// #SVN_FS_CONFIG_FS_TYPE then the default filesystem type will be used.
    /// This will typically be BDB for version 1.1 and FSFS for later versions,
    /// though the caller should not rely upon any particular default if they
    /// wish to ensure that a filesystem of a specific type is created.
    ///
    /// `svn_fs`/`svn_fs_create2`
    pub fn create(
        fs: &mut Option<Rc<dyn FsInstance>>,
        db_path: &Path,
        config: &FsConfig,
    ) -> Result<Self, Error> {
        let fs_type = config.fs_type();

        let new_fs: Box<dyn FsInstance> = match fs_type {
            #[allow(deprecated)]
            FsType::Bdb => {
                return Err(Error::UnsupportedFsType(FsType::Bdb));
            }
            FsType::Fsfs => {
                // Create a new FSFS filesystem
                Box::new(FsFsBackend::new(db_path.to_path_buf()))
            }
            FsType::Fsx => {
                return Err(Error::UnsupportedFsType(FsType::Fsx));
            }
        };

        // Create the FS directory and write out the fsap-name file.
        if !db_path.exists() {
            fs_err::create_dir_all(db_path)?;
        }
        // write fs-type file
        let fs_type_file = db_path.join(FS_TYPE_FILENAME);
        fs_err::write(fs_type_file, format!("{}\n", fs_type.to_string()))?;

        // let ret = Self {
        //     path:
        // }

        // Perform the actual creation
        new_fs.create(db_path)?;
        // new_fs.open(db_path, config)?;
        Ok(ret)
    }

    /// Open a Subversion filesystem located in the directory @a path, and
    /// return a pointer to it in @a *fs_p.  If @a fs_config is not @c
    /// NULL, the options it contains modify the behavior of the
    /// filesystem.  The interpretation of @a fs_config is specific to the
    /// filesystem back-end.  The opened filesystem will be allocated in
    /// @a result_pool may be closed by clearing or destroying that pool.
    /// Use @a scratch_pool for temporary allocations.
    ///
    /// @note The lifetime of @a fs_config must not be shorter than @a
    /// result_pool's. It's a good idea to allocate @a fs_config from
    /// @a result_pool or one of its ancestors.
    ///
    /// Only one thread may operate on any given filesystem object at once.
    /// Two threads may access the same filesystem simultaneously only if
    /// they open separate filesystem objects.
    ///
    /// @note You probably don't want to use this directly.  
    /// Take a look at svn_repos_open3() `Repos::open` instead.
    ///
    /// `svn_fs_open2`
    fn fs_open(&mut self, path: &Path, config: &FsConfig) -> Result<(), Error> {
        let fs_lib = self.fs_library();

        fs_lib.open_fs(path)?;

        Ok(())
    }

    /// `fs_library_vtable`
    fn fs_library(&self) -> &dyn FsLibrary {
        let ins = self.inner.as_ref().unwrap();
        let fs_lib: &dyn FsLibrary = ins.as_ref();
        fs_lib
    }
}
