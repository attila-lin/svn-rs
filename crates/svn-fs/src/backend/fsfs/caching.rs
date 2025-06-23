//! `subversion\libsvn_fs_fs\caching.c`

use svn_subr::SvnConfig;

use crate::SvnFs;

/// Select the cache namespace.  If you potentially share the cache with
/// another FS object for the same repository, objects read through one FS
/// will not need to be read again for the other.  In most cases, that is
/// a very desirable behavior and the default is, therefore, an empty
/// namespace.
///
/// If you want to be sure that your FS instance will actually read all
/// requested data at least once, you need to specify a separate namespace
/// for it.  All repository verification code, for instance, should use
/// some GUID here that is different each time you open an FS instance.
///
/// @since New in 1.8.
///
const SVN_FS_CONFIG_FSFS_CACHE_NS: &str = "fsfs-cache-namespace";

/// Enable / disable text delta caching for a FSFS repository.
///
/// @since New in 1.7.
///
const SVN_FS_CONFIG_FSFS_CACHE_DELTAS: &str = "fsfs-cache-deltas";

/// Enable / disable full-text caching for a FSFS repository.
///
/// @since New in 1.7.
///
const SVN_FS_CONFIG_FSFS_CACHE_FULLTEXTS: &str = "fsfs-cache-fulltexts";

/// Enable / disable caching of node properties for a FSFS repository.
///
/// @since New in 1.10.
const SVN_FS_CONFIG_FSFS_CACHE_NODEPROPS: &str = "fsfs-cache-nodeprops";

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum CachingError {
    #[error(transparent)]
    ParseBool(#[from] std::str::ParseBoolError),
}

/// *CACHE_TXDELTAS, *CACHE_FULLTEXTS, *CACHE_NODEPROPS flags will be set
/// according to FS->CONFIG. *CACHE_NAMESPACE receives the cache prefix to
/// use.
///
/// Use FS->pool for allocating the memcache and CACHE_NAMESPACE, and POOL
/// for temporary allocations.
///
/// `read_config`
pub fn read_config(fs: &SvnFs) -> Result<(String, bool, bool, bool), CachingError> {
    //  No cache namespace by default.  I.e. all FS instances share the
    // cached data.  If you specify different namespaces, the data will
    // share / compete for the same cache memory but keys will not match
    // across namespaces and, thus, cached data will not be shared between
    // namespaces.
    //
    // Since the namespace will be concatenated with other elements to form
    // the complete key prefix, we must make sure that the resulting string
    let cache_namespace = fs
        .config()
        .get(SVN_FS_CONFIG_FSFS_CACHE_NS)
        .cloned()
        .unwrap_or_default();

    // Cache text deltas by default.
    // They tend to be smaller and have finer granularity than fulltexts.
    let cache_txdeltas =
        svn_subr::hash::get_bool(fs.config(), SVN_FS_CONFIG_FSFS_CACHE_DELTAS, true);

    // by default, cache fulltexts.
    // Most SVN tools care about reconstructed file content.
    // Thus, this is a reasonable default.
    // SVN admin tools may set that to FALSE because fulltexts
    // won't be re-used rendering the cache less effective
    // by squeezing wanted data out.
    let cache_fulltexts =
        svn_subr::hash::get_bool(fs.config(), SVN_FS_CONFIG_FSFS_CACHE_FULLTEXTS, true);

    //  by default, cache nodeprops.
    // Pre-1.10, this was controlled by the SVN_FS_CONFIG_FSFS_CACHE_FULLTEXTS
    // configuration option which defaulted to TRUE.
    let cache_nodeprops =
        svn_subr::hash::get_bool(&fs.config(), SVN_FS_CONFIG_FSFS_CACHE_NODEPROPS, true);

    Ok((
        cache_namespace,
        cache_txdeltas,
        cache_fulltexts,
        cache_nodeprops,
    ))
}
