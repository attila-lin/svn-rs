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
use super::SVN_FS_CONFIG_FSFS_SHARD_SIZE;
use crate::SvnFs;
use crate::backend::PATH_FORMAT;
use crate::backend::PATH_REV;
use crate::backend::PATH_REVS_DIR;
use crate::backend::PATH_UUID;
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
    //
    // `fs_create`
    fn create(fs: &mut SvnFs, path: &Path) -> Result<(), BackendError> {
        let data = Self::initialize_fs_struct();

        Ok(())
    }

    /// This implements the [`FsLibrary`].[`open_fs`]() API.  Open an FSFS
    /// Subversion filesystem located at PATH, set *FS to point to the
    /// correct vtable for the filesystem.  Use POOL for any temporary
    /// allocations, and COMMON_POOL for fs-global allocations.
    /// The latter must be serialized using COMMON_POOL_LOCK.
    ///
    /// `fs_open`
    fn open_fs(&self, path: &Path) -> Result<(), BackendError> {
        // TODO:
        // self.check_fs(path)?;

        Self::initialize_fs_struct();

        self._open(path)?;
        self._initialize_cache()?;

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
    fn _create(fs: &mut SvnFs, path: &Path) -> Result<(), BackendError> {
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
        let mut f = FsFsBackend::new(path.to_path_buf());
        f.create_file_tree(path, format, shard_size, log_addressing)?;

        // This filesystem is ready.  Stamp it with a format number.
        s.write_format()
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
            fs_err::write(&format_path, sb)?;
        } else {
            svn_subr::io::write_atomic(&self.path, sb.as_bytes(), None, ffd.flush_to_disk)?;
        }

        svn_subr::io::set_file_read_only(&format_path, false)?;
        Ok(())
    }

    ///  Open the fsfs filesystem pointed to by PATH and associate it with
    ///    filesystem object FS.  Use POOL for temporary allocations.
    ///
    ///    ### Some parts of *FS must have been initialized beforehand; some parts
    ///       (including FS->path) are initialized by this function.
    /// `svn_fs_fs__open`
    fn _open(&mut self, path: &Path) -> Result<(), BackendError> {
        self.path = path.to_path_buf();

        // Read the FS format file.
        self._read_format_file(path)?;

        // Read in and cache the repository uuid.
        self._read_uuid(path)?;

        Ok(())
    }

    /// Read the 'format' file of fsfs filesystem FS and store its info in FS.
    ///
    /// `svn_fs_fs__read_format_file`
    fn _read_format_file(&mut self, path: &Path) -> Result<(), BackendError> {
        let ffd = self._data_mut();
        let format_path = path.join(PATH_FORMAT);

        // Read info from format file.
        let (format, max_files_per_dir, use_log_addressing) = Self::_read_format(&format_path)?;

        //  Now that we've got *all* info, store / update values in FFD.
        ffd.format = format;
        ffd.max_files_per_dir = max_files_per_dir;
        ffd.use_log_addressing = use_log_addressing;

        Ok(())
    }

    /// Read the format number and maximum number of files per directory
    ///   from PATH and return them in *PFORMAT, *MAX_FILES_PER_DIR and
    ///   USE_LOG_ADDRESSIONG respectively.
    ///
    ///   *MAX_FILES_PER_DIR is obtained from the 'layout' format option, and
    ///   will be set to zero if a linear scheme should be used.
    ///   *USE_LOG_ADDRESSIONG is obtained from the 'addressing' format option,
    ///   and will be set to FALSE for physical addressing.
    /// `read_format`
    fn _read_format(format_path: &Path) -> Result<(u32, u32, bool), BackendError> {
        let mut format = 0;
        let mut max_files_per_dir = 0;
        let mut use_log_addressing = false;

        // Read the format file and parse the values.
        if format_path.exists() {
            let content = fs_err::read_to_string(format_path)?;
            for line in content.lines() {
                if line.starts_with("layout sharded") {
                    max_files_per_dir = line
                        .split_whitespace()
                        .nth(2)
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(0);
                } else if line.starts_with("layout linear") {
                    max_files_per_dir = 0;
                } else if line.starts_with("addressing logical") {
                    use_log_addressing = true;
                } else if line.starts_with("addressing physical") {
                    use_log_addressing = false;
                } else if let Ok(num) = line.parse::<u32>() {
                    format = num;
                }
            }
        }

        Ok((format, max_files_per_dir, use_log_addressing))
    }

    /// Read FS's UUID file and store the data in the FS struct.
    /// `read_uuid`
    fn _read_uuid(&mut self, path: &Path) -> Result<(), BackendError> {
        let uuid_path = path.join(PATH_UUID);
        if !uuid_path.exists() {
            return Err(BackendError::FileNotFound(
                uuid_path.to_string_lossy().to_string(),
            ));
        }

        let uuid = fs_err::read_to_string(uuid_path)?;
        self._data_mut().uuid = uuid.trim().to_string();

        Ok(())
    }
}
