// copy from `subversion/libsvn_fs_fs/fs.c`

use std::fmt::Write;
use std::path::Path;
use std::path::PathBuf;

use svn_types::RevisionNumber;

use super::super::BackendError;
use super::super::FsInstance;
use super::super::FsLibrary;
use super::FORMAT_NUMBER;
use super::FsFsData;
use crate::SvnFs;
use crate::backend::PATH_FORMAT;
use crate::backend::PATH_REV;
use crate::backend::PATH_REVS_DIR;
use crate::backend::fsfs::FsFsBackend;
use crate::backend::fsfs::SVN_FS_CONFIG_FSFS_LOG_ADDRESSING;

impl FsLibrary for FsFsBackend {
    fn get_version(&self, path: &Path) -> Result<(), BackendError> {
        todo!()
    }

    // This implements the fs_library_vtable_t.create() API.  Create a new
    // fsfs-backed Subversion filesystem at path PATH and link it into
    // *FS.  Perform temporary allocations in POOL, and fs-global allocations
    // in COMMON_POOL.  The latter must be serialized using COMMON_POOL_LOCK.
    // `fs_create`
    fn create(&self, path: &Path) -> Result<(), BackendError> {
        let data = Self::initialize_fs_struct();
    }
    fn open_fs(&self, path: &Path) -> Result<(), BackendError> {
        todo!()
    }
    fn open_fs_for_recovery(&self, path: &str) -> Result<(), BackendError> {
        todo!()
    }
    fn upgrade_fs(&self, path: &str) -> Result<(), BackendError> {
        todo!()
    }
    fn verify_fs(&self, path: &str) -> Result<(), BackendError> {
        todo!()
    }
    fn delete_fs(&self, path: &str) -> Result<(), BackendError> {
        todo!()
    }
    fn hotcopy(&self, src_path: &str, dst_path: &str) -> Result<(), BackendError> {
        todo!()
    }
    fn pack_fs(&self, path: &str) -> Result<(), BackendError> {
        todo!()
    }
}

impl FsFsBackend {
    /// Set up vtable and fsap_data fields in FS.
    /// `initialize_fs_struct`
    fn initialize_fs_struct() {
        let mut data = FsFsData::default();
        data.flush_to_disk = true;
    }

    // `svn_fs_fs__create`
    fn _create(fs: &mut SvnFs, path: &Path) {
        // We don't care version, just use 8
        let format = FORMAT_NUMBER;

        let shard_size = if let Some(share_size_str) = fs.config.get(SVN_FS_CONFIG_FSFS_SHARD_SIZE)
        {
            share_size_str.parse::<u32>().unwrap_or(0)
        } else {
            0
        };

        let log_addressing: bool = fs
            .config
            .get(SVN_FS_CONFIG_FSFS_LOG_ADDRESSING)
            .map_or(false, |v| v == "true");

        // Actual FS creation.
        Self::create_file_tree(path, format, shard_size, log_addressing)?;

        // This filesystem is ready.  Stamp it with a format number.
        self.write_format()
    }

    /// Under the repository db PATH, create a FSFS repository with FORMAT,
    /// the given SHARD_SIZE. If USE_LOG_ADDRESSING is non-zero, repository
    /// will use logical addressing. If not supported by the respective format,
    /// the latter two parameters will be ignored. FS will be updated.
    ///
    /// The only file not being written is the 'format' file.  This allows
    /// callers such as hotcopy to modify the contents before turning the
    /// tree into an accessible repository.
    /// `svn_fs_fs__create_file_tree`
    fn create_file_tree(
        &mut self,
        path: &Path,
        format: u32,
        shard_size: u32,
        use_log_addressing: bool,
    ) -> Result<(), BackendError> {
        let ffd = self._data_mut();

        // FIXME:
        // self.path = path.to_path_buf();
        ffd.format = format;

        // Use an appropriate sharding mode if supported by the format.
        ffd.max_files_per_dir = if format >= 3 { shard_size } else { 0 };

        //  Create the revision data directories.
        if ffd.max_files_per_dir > 0 {
            let p = self.path_rev_shard(0);
            fs_err::create_dir_all(p)?;
        } else {
            let p = path.join(PATH_REVS_DIR);
            fs_err::create_dir_all(p)?;
        }

        Ok(())
    }
}

/// utils
impl FsFsBackend {
    /// Return the full path of the rev shard directory that will contain
    /// revision REV in FS.
    ///
    /// `svn_fs_fs__path_rev_shard`
    fn path_rev_shard(&self, rev: RevisionNumber) -> PathBuf {
        let ffd = self._data();

        assert!(ffd.max_files_per_dir > 0, "max_files_per_dir must be set");
        self.path
            .join(PATH_REVS_DIR)
            .join(format!("{:0>8}", rev / ffd.max_files_per_dir as i64))
    }

    /// Return the full path of the non-packed rev file containing revision REV in FS.
    fn path_rev(&self, rev: u32) -> PathBuf {
        todo!()
    }

    /// Return TRUE is REV is packed in FS, FALSE otherwise.
    ///
    /// `svn_fs_fs__is_packed_rev`
    fn is_packed_rev(&self, rev: RevisionNumber) -> bool {
        let ffd = self._data();
        rev < ffd.min_unpacked_rev
    }

    ///  Write the format number, maximum number of files per directory and
    ///    the addressing scheme to a new format file in PATH, possibly expecting
    ///    to overwrite a previously existing file.
    fn write_format(&mut self, overwrite: bool) -> Result<(), BackendError> {
        let ffd = self._data();
        let format_path = self.path.join(PATH_FORMAT);

        assert!(
            1 <= ffd.format && ffd.format <= FORMAT_NUMBER,
            "format number must be between 1 and {}",
            FORMAT_NUMBER
        );

        let mut sb = String::new();
        sb.write_str(&format!("{}\n", ffd.format))?;

        if ffd.format > 3 {
            if ffd.max_files_per_dir > 0 {
                sb.write_str(&format!("layout sharded {}\n", ffd.max_files_per_dir))?;
            } else {
                sb.write_str("layout linear\n")?;
            }
        }
        if ffd.format >= 7 {
            if ffd.use_log_addressing {
                sb.write_str("addressing logical\n")?;
            } else {
                sb.write_str("addressing physical\n")?;
            }
        }

        // svn_io_write_version_file() does a load of magic to allow it to
        //  replace version files that already exist.  We only need to do
        //  that when we're allowed to overwrite an existing file.
        if !overwrite {
            // create the file
            if format_path.exists() {
                return Err(BackendError::FileExists(
                    format_path.to_string_lossy().to_string(),
                ));
            }
            fs_err::write(format_path, sb)?;
        } else {
            svn_subr::io::write_atomic(path, sb, None, ffd.flush_to_disk)?;
        }

        todo!()
    }
}
