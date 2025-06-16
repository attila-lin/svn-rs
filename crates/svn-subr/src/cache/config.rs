//! `subversion\libsvn_subr\cache_config.c`
//!
//! configuration of internal caches

/// Cache resource settings. It controls what caches, in what size and
/// how they will be created. The settings apply for the whole process.
///
/// @note Do not extend this data structure as this would break binary
/// compatibility.
///
/// `svn_cache_config_t`
pub struct CacheConfig {
    /// total cache size in bytes. Please note that this is only soft limit
    /// to the total application memory usage and will be exceeded due to
    /// temporary objects and other program state.
    /// May be 0, resulting in default caching code being used.
    cache_size: u64,

    /// maximum number of files kept open
    file_handle_count: usize,

    ///  is this application guaranteed to be single-threaded?
    single_threaded: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            // 16 MB for caches.
            // If you are running a single server process,
            // you may easily increase that to 50+% of your RAM
            // using svn_fs_set_cache_config().
            cache_size: 0x1000000,
            // up to 16 files kept open.
            // Most OS restrict the number of open file handles to
            // about 1000. To minimize I/O and OS overhead, values
            // of 500+ can be beneficial (use svn_fs_set_cache_config()
            // to change the configuration).
            // When running with a huge in-process cache, this number
            // has little impact on performance and a more modest
            // value (< 100) may be more suitable.
            file_handle_count: 16,

            single_threaded: false,
        }
    }
}
