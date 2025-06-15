use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use uuid::Uuid;

use crate::Error;
use crate::FsType;
use crate::backend::FsInstance;
use crate::backend::fsfs::{FsFsBackend, FsFsData};
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
    access_ctx: FsAccess,

    // **Note:** Moved to [`inner`]
    // /// FSAP-specific vtable and private data
    // ///
    // /// FIXME: now use [`FsFsData`]
    // data: Option<FsFsData>,
    /// The filesystem library that implements this filesystem
    inner: Option<Box<dyn FsInstance>>,

    /// UUID, stored by open(), create(), and set_uuid().
    uuid: Uuid,
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
    /// `svn_fs_create2`
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

        // Perform the actual creation
        new_fs.create(db_path, config)?;
        new_fs.open(db_path, config)?;
        todo!()
    }
}
