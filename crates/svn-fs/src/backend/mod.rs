//! diff backend for svn-fs

mod error;
use std::path::Path;

pub use error::BackendError;

pub mod fsfs;

pub mod fsx;

// Names of special files in the fs_fs filesystem
// from `fs.h`

/// Contains format number
pub const PATH_FORMAT: &str = "format";
/// Contains UUID
pub const PATH_UUID: &str = "uuid";
/// Youngest revision
pub const PATH_CURRENT: &str = "current";
/// Revision lock file
pub const PATH_LOCK_FILE: &str = "write-lock";
/// Pack lock file
pub const PATH_PACK_LOCK_FILE: &str = "pack-lock";

// Directories in the fs_fs filesystem

/// Directory of revisions
pub const PATH_REVS_DIR: &str = "revs";
/// Directory of revprops
pub const PATH_REVPROPS_DIR: &str = "revprops";
/// Directory of transactions in repos w/o log addressing
pub const PATH_TXNS_DIR: &str = "transactions";
/// Lazy node-origin cache
pub const PATH_NODE_ORIGINS_DIR: &str = "node-origins";

/// Directory of proto-revs
pub const PATH_TXN_PROTOS_DIR: &str = "txn-protorevs";

///  File with next txn key
pub const PATH_TXN_CURRENT: &str = "txn-current";

/// Lock for txn-current
pub const PATH_TXN_CURRENT_LOCK: &str = "txn-current-lock";
/// Directory of locks
pub const PATH_LOCKS_DIR: &str = "locks";
/// Oldest revision which has not been packed.
pub const PATH_MIN_UNPACKED_REV: &str = "min-unpacked-rev";
/// Current revprop generation
pub const PATH_REVPROP_GENERATION: &str = "revprop-generation";

/// Manifest file name
pub const PATH_MANIFEST: &str = "manifest";
///  Packed revision data file
pub const PATH_PACKED: &str = "pack";
/// Extension for packed shards
pub const PATH_EXT_PACKED_SHARD: &str = ".pack";
/// extension of the log-to-phys index
pub const PATH_EXT_L2P_INDEX: &str = ".l2p";
/// extension of the phys-to-log index
pub const PATH_EXT_P2L_INDEX: &str = ".p2l";
/* If you change this, look at tests/svn_test_fs.c(maybe_install_fsfs_conf) */
// **NOTE:** moved to fsfs module
// #define PATH_CONFIG           "fsfs.conf"        /* Configuration */
/* Names of special files and file extensions for transactions */

///  Records changes made so far
pub const PATH_CHANGES: &str = "changes";
///  Transaction properties
pub const PATH_TXN_PROPS: &str = "props";
/// Next temporary ID assignments
pub const PATH_NEXT_IDS: &str = "next-ids";
///  Prefix for node filename
pub const PATH_PREFIX_NODE: &str = "node.";
/// Extension of txn dir
pub const PATH_EXT_TXN: &str = ".txn";
/// Extension for dir contents
pub const PATH_EXT_CHILDREN: &str = ".children";
///  Extension for node props
pub const PATH_EXT_PROPS: &str = ".props";
///  Extension of protorev file
pub const PATH_EXT_REV: &str = ".rev";
///  Extension of protorev lock file
pub const PATH_EXT_REV_LOCK: &str = ".rev-lock";
///  File containing the current item index number
pub const PATH_TXN_ITEM_INDEX: &str = "itemidx";
/// name of index files w/o ext
pub const PATH_INDEX: &str = "index";

/* Names of files in legacy FS formats */
/// Proto rev file
pub const PATH_REV: &str = "rev";
/// Proto rev (write) lock file
pub const PATH_REV_LOCK: &str = "rev-lock";

/* Names of sections and options in fsfs.conf. */
// FIXME: make it struct

pub const CONFIG_SECTION_CACHES: &str = "caches";
pub const CONFIG_OPTION_FAIL_STOP: &str = "fail-stop";
pub const CONFIG_SECTION_REP_SHARING: &str = "rep-sharing";
pub const CONFIG_OPTION_ENABLE_REP_SHARING: &str = "enable-rep-sharing";
pub const CONFIG_SECTION_DELTIFICATION: &str = "deltification";
pub const CONFIG_OPTION_ENABLE_DIR_DELTIFICATION: &str = "enable-dir-deltification";
pub const CONFIG_OPTION_ENABLE_PROPS_DELTIFICATION: &str = "enable-props-deltification";
pub const CONFIG_OPTION_MAX_DELTIFICATION_WALK: &str = "max-deltification-walk";
pub const CONFIG_OPTION_MAX_LINEAR_DELTIFICATION: &str = "max-linear-deltification";
pub const CONFIG_OPTION_COMPRESSION_LEVEL: &str = "compression-level";
pub const CONFIG_SECTION_PACKED_REVPROPS: &str = "packed-revprops";
pub const CONFIG_OPTION_REVPROP_PACK_SIZE: &str = "revprop-pack-size";
pub const CONFIG_OPTION_COMPRESS_PACKED_REVPROPS: &str = "compress-packed-revprops";
pub const CONFIG_SECTION_IO: &str = "io";
pub const CONFIG_OPTION_BLOCK_SIZE: &str = "block-size";
pub const CONFIG_OPTION_L2P_PAGE_SIZE: &str = "l2p-page-size";
pub const CONFIG_OPTION_P2L_PAGE_SIZE: &str = "p2l-page-size";
pub const CONFIG_SECTION_DEBUG: &str = "debug";
pub const CONFIG_OPTION_PACK_AFTER_COMMIT: &str = "pack-after-commit";
pub const CONFIG_OPTION_VERIFY_BEFORE_COMMIT: &str = "verify-before-commit";
pub const CONFIG_OPTION_COMPRESSION: &str = "compression";

use svn_types::RevisionNumber;

/// vtable types for the abstract FS objects
///
/// `fs_vtable_t`
pub trait FsInstance: FsLibrary {
    fn youngest_rev(&self) -> RevisionNumber;
    fn refresh_revision_prop(&self) -> Result<(), BackendError>;

    fn revision_prop(&self) -> Result<(), BackendError>;
}

/// Top-level library vtable type
/// `fs_library_vtable_t`
pub trait FsLibrary: Send + Sync {
    /// This field should always remain first in the vtable.
    /// Apart from that, it can be changed however you like, since exact
    /// version equality is required between loader and module.  This policy
    /// was weaker during 1.1.x, but only in ways which do not conflict with
    /// this statement, now that the minor version has increased.
    fn get_version(&self, path: &Path) -> Result<(), BackendError>;

    /// The open_fs/create/open_fs_for_recovery/upgrade_fs functions must
    /// use the common_pool_lock to serialize the access to the common_pool
    /// parameter for allocating fs-global objects such as an env cache.
    fn create(&self, path: &Path) -> Result<(), BackendError>;
    fn open_fs(&self, path: &Path) -> Result<(), BackendError>;
    fn open_fs_for_recovery(&self, path: &str) -> Result<(), BackendError>;
    fn upgrade_fs(&self, path: &str) -> Result<(), BackendError>;
    fn verify_fs(&self, path: &str) -> Result<(), BackendError>;
    fn delete_fs(&self, path: &str) -> Result<(), BackendError>;
    fn hotcopy(&self, src_path: &str, dst_path: &str) -> Result<(), BackendError>;
    fn pack_fs(&self, path: &str) -> Result<(), BackendError>;
}
