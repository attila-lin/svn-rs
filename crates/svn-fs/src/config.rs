//! port `Filesystem configuration options`

use uuid::Uuid;

use crate::Error;
use crate::SvnFs;

/// `svn_fs_type`
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsType {
    #[deprecated]
    Ddb,
    Fsfs,
    /// [EXPERIMENTAL] filesystem backend.
    ///
    /// It is not ready for general production use.  Please consult the
    /// respective release notes on suggested usage scenarios.
    Fsx,
}

#[derive(Debug, Clone)]
pub enum FsConfig {
    Fsfs(FsFsConfig),
}

impl Default for FsConfig {
    fn default() -> Self {
        FsConfig::Fsfs(FsFsConfig::default())
    }
}

impl FsConfig {
    pub fn fs_type(&self) -> FsType {
        match self {
            FsConfig::Fsfs(_) => FsType::Fsfs,
        }
    }
}

/// Filesystem configuration options for a `FSFS` repository.
#[derive(Debug, Default, Clone)]
pub struct FsFsConfig {
    /// Enable / disable text delta caching for a FSFS repository.
    pub cache_deltas: bool,
    /// Enable / disable full-text caching for a FSFS repository.
    pub cache_fulltexts: bool,
    /// Enable / disable revprop caching for a FSFS repository.
    //  *
    //  * "2" is allowed, too and means "enable if efficient",
    //  * i.e. this will not create warning at runtime if there
    //  * is no efficient support for revprop caching.
    pub cache_revprops: bool,
    /// Select the cache namespace.  If you potentially share the cache with
    //  * another FS object for the same repository, objects read through one FS
    //  * will not need to be read again for the other.  In most cases, that is
    //  * a very desirable behavior and the default is, therefore, an empty
    //  * namespace.
    //  *
    //  * If you want to be sure that your FS instance will actually read all
    //  * requested data at least once, you need to specify a separate namespace
    //  * for it.  All repository verification code, for instance, should use
    //  * some GUID here that is different each time you open an FS instance.
    pub cache_namespace: Uuid,
    /// Enable / disable the FSFS format 7 "block read" feature.
    pub block_read: bool,
    /// String with a decimal representation of the FSFS format shard size.
    //  * Zero ("0") means that a repository with linear layout should be created.
    //  *
    //  * This option will only be used during the creation of new repositories
    //  * and is otherwise ignored.
    pub shard_size: String,

    /// String with a decimal representation of the FSFS format shard size.
    //  * Zero ("0") means that a repository with linear layout should be created.
    //  *
    //  * This option will only be used during the creation of new repositories
    //  * and is otherwise ignored.
    pub log_addressing: bool,
    /// Specifies whether the filesystem should be forcing a physical write of
    //  * the data to disk.  Enabling the option allows the filesystem to return
    //  * from the API calls without forcing the write to disk.  If this option
    //  * is disabled, the changes are always written to disk.
    //  *
    //  * @note Avoiding the forced write to disk usually is more efficient, but
    //  * doesn't guarantee data integrity after a system crash or power failure
    //  * and should be used with caution.
    pub no_flush_to_disk: bool,
}

impl FsFsConfig {
    const PATH_CONFIG: &'static str = "fsfs.conf";

    /// Write FS' initial configuration file.
    ///
    /// `write_config`
    pub fn write_config(fs: &SvnFs) -> Result<(), Error> {
        let content = include_str!("../config/fsfs.conf");
        let path = fs.path.join(Self::PATH_CONFIG);

        std::fs::write(&path, content)?;

        Ok(())
    }
}
