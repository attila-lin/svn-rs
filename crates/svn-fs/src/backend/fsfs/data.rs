use crate::CompressionType;
use svn_subr::MemCache;
use svn_subr::SvnCache;
use svn_types::RevisionNumber;

/// Private (non-shared) FSFS-specific data for each svn_fs_t object.
/// Any caches in here may be NULL.
///
/// `fs_fs_data_t`
#[derive(Debug, Default)]
pub struct FsFsData {
    /// The format number of this FS.
    pub format: u32,
    /// The maximum number of files to store per directory (for sharded
    /// layouts) or zero (for linear layouts).
    pub max_files_per_dir: u32,

    /// If set, this FS is using logical addressing.
    /// Otherwise, it is using physical addressing.
    pub use_log_addressing: bool,

    /// Rev / pack file read granularity in bytes.
    pub block_size: i64,

    /// Capacity in entries of log-to-phys index pages
    pub l2p_page_size: i64,

    /// Rev / pack file granularity (in bytes) covered by a single phys-to-log
    /// index page.
    pub p2l_page_size: i64,

    /// If set, parse and cache *all* data of each block that we read
    /// (not just the one bit that we need, atm).
    pub use_block_read: bool,

    /// The revision that was youngest, last time we checked.
    pub youngest_rev_cache: RevisionNumber,

    // Caches of immutable data.  (Note that these may be shared between
    // multiple svn_fs_t's for the same filesystem.)
    /// Access to the configured memcached instances.  May be NULL.
    pub memcache: Option<MemCache>,

    /// If TRUE, don't ignore any cache-related errors.  If FALSE, errors from
    /// e.g. memcached may be ignored as caching is an optional feature.
    pub fail_stop: bool,

    // TODO: move to `FsFsDataCache`
    /// A cache of revision root IDs, mapping from (svn_revnum_t *) to
    /// (svn_fs_id_t *).  (Not threadsafe.)
    pub rev_root_id_cache: Option<SvnCache<(), ()>>,

    /// Pack after every commit.
    pack_after_commit: bool,

    /// Verify each new revision before commit.
    verify_before_commit: bool,

    /// Per-instance filesystem ID, which provides an additional level of
    /// uniqueness for filesystems that share the same UUID, but should
    /// still be distinguishable (e.g. backups produced by svn_fs_hotcopy()
    /// or dump / load cycles).
    pub instance_id: Option<String>,

    /// Ensure that all filesystem changes are written to disk.
    pub(crate) flush_to_disk: bool,

    /// The oldest revision not in a pack file.  It also applies to revprops
    /// if revprop packing has been enabled by the FSFS format version.
    pub min_unpacked_rev: RevisionNumber,

    /// Whether rep-sharing is supported by the filesystem and allowed by the configuration.
    pub rep_sharing_allowed: bool,

    /// File size limit in bytes up to which multiple revprops shall be packed into a single file.
    pub revprop_pack_size: i64,

    /// Whether packed revprop files shall be compressed.
    pub compress_packed_revprops: bool,

    // TODO: move to deltify_data
    /// Whether directory nodes shall be deltified just like file nodes.
    pub deltify_directories: bool,

    /// Whether nodes properties shall be deltified.
    pub deltify_properties: bool,

    /// Restart deltification histories after each multiple of this value
    pub max_deltification_walk: i64,

    /// Maximum number of length of the linear part at the top of the
    /// deltification history after which skip deltas will be used.
    pub max_linear_deltification: i64,

    /// Compression type to use with txdelta storage format in new revs.
    pub delta_compression_type: CompressionType,

    /// cache
    cache: FsFsDataCache,
}

/// `fs_fs_data_t`'s inner cache
#[derive(Debug, Default)]
pub struct FsFsDataCache {}
