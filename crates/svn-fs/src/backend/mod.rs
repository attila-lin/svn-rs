//! diff backend for svn-fs

pub mod fsfs;

pub mod fsx;

// Names of special files in the fs_fs filesystem
// from `fs.h`

/// Contains format number
const PATH_FORMAT: &str = "format";
/// Contains UUID
const PATH_UUID: &str = "uuid";
/// Youngest revision
const PATH_CURRENT: &str = "current";
/// Revision lock file
const PATH_LOCK_FILE: &str = "write-lock";
/// Pack lock file
const PATH_PACK_LOCK_FILE: &str = "pack-lock";

// Directories in the fs_fs filesystem

/// Directory of revisions
const PATH_REVS_DIR: &str = "revs";
/// Directory of revprops
const PATH_REVPROPS_DIR: &str = "revprops";
/// Directory of transactions in repos w/o log addressing
const PATH_TXNS_DIR: &str = "transactions";
/// Lazy node-origin cache
const PATH_NODE_ORIGINS_DIR: &str = "node-origins";

/// Directory of proto-revs
const PATH_TXN_PROTOS_DIR: &str = "txn-protorevs";

///  File with next txn key
const PATH_TXN_CURRENT: &str = "txn-current";

/// Lock for txn-current
const PATH_TXN_CURRENT_LOCK: &str = "txn-current-lock";
/// Directory of locks
const PATH_LOCKS_DIR: &str = "locks";
/// Oldest revision which has not been packed.
const PATH_MIN_UNPACKED_REV: &str = "min-unpacked-rev";
/// Current revprop generation
const PATH_REVPROP_GENERATION: &str = "revprop-generation";

/// Manifest file name
const PATH_MANIFEST: &str = "manifest";
///  Packed revision data file
const PATH_PACKED: &str = "pack";
/// Extension for packed shards
const PATH_EXT_PACKED_SHARD: &str = ".pack";
/// extension of the log-to-phys index
const PATH_EXT_L2P_INDEX: &str = ".l2p";
/// extension of the phys-to-log index
const PATH_EXT_P2L_INDEX: &str = ".p2l";
/* If you change this, look at tests/svn_test_fs.c(maybe_install_fsfs_conf) */
// **NOTE:** moved to fsfs module
// #define PATH_CONFIG           "fsfs.conf"        /* Configuration */
/* Names of special files and file extensions for transactions */

///  Records changes made so far
const PATH_CHANGES: &str = "changes";
///  Transaction properties
const PATH_TXN_PROPS: &str = "props";
/// Next temporary ID assignments
const PATH_NEXT_IDS: &str = "next-ids";
///  Prefix for node filename
const PATH_PREFIX_NODE: &str = "node.";
/// Extension of txn dir
const PATH_EXT_TXN: &str = ".txn";
/// Extension for dir contents
const PATH_EXT_CHILDREN: &str = ".children";
///  Extension for node props
const PATH_EXT_PROPS: &str = ".props";
///  Extension of protorev file
const PATH_EXT_REV: &str = ".rev";
///  Extension of protorev lock file
const PATH_EXT_REV_LOCK: &str = ".rev-lock";
///  File containing the current item index number
const PATH_TXN_ITEM_INDEX: &str = "itemidx";
/// name of index files w/o ext
const PATH_INDEX: &str = "index";

/* Names of files in legacy FS formats */
/// Proto rev file
const PATH_REV: &str = "rev";
/// Proto rev (write) lock file
const PATH_REV_LOCK: &str = "rev-lock";

/* Names of sections and options in fsfs.conf. */
// FIXME: make it struct

const CONFIG_SECTION_CACHES: &str = "caches";
const CONFIG_OPTION_FAIL_STOP: &str = "fail-stop";
const CONFIG_SECTION_REP_SHARING: &str = "rep-sharing";
const CONFIG_OPTION_ENABLE_REP_SHARING: &str = "enable-rep-sharing";
const CONFIG_SECTION_DELTIFICATION: &str = "deltification";
const CONFIG_OPTION_ENABLE_DIR_DELTIFICATION: &str = "enable-dir-deltification";
const CONFIG_OPTION_ENABLE_PROPS_DELTIFICATION: &str = "enable-props-deltification";
const CONFIG_OPTION_MAX_DELTIFICATION_WALK: &str = "max-deltification-walk";
const CONFIG_OPTION_MAX_LINEAR_DELTIFICATION: &str = "max-linear-deltification";
const CONFIG_OPTION_COMPRESSION_LEVEL: &str = "compression-level";
const CONFIG_SECTION_PACKED_REVPROPS: &str = "packed-revprops";
const CONFIG_OPTION_REVPROP_PACK_SIZE: &str = "revprop-pack-size";
const CONFIG_OPTION_COMPRESS_PACKED_REVPROPS: &str = "compress-packed-revprops";
const CONFIG_SECTION_IO: &str = "io";
const CONFIG_OPTION_BLOCK_SIZE: &str = "block-size";
const CONFIG_OPTION_L2P_PAGE_SIZE: &str = "l2p-page-size";
const CONFIG_OPTION_P2L_PAGE_SIZE: &str = "p2l-page-size";
const CONFIG_SECTION_DEBUG: &str = "debug";
const CONFIG_OPTION_PACK_AFTER_COMMIT: &str = "pack-after-commit";
const CONFIG_OPTION_VERIFY_BEFORE_COMMIT: &str = "verify-before-commit";
const CONFIG_OPTION_COMPRESSION: &str = "compression";
