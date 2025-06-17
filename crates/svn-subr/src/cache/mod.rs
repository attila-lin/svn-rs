//! `svn_cache__t`

mod memcache;
pub use memcache::MemCache;

pub mod config;

use std::fmt::Debug;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("cache full")]
    CacheFull,
    #[error("key not found")]
    KeyNotFound,
    #[error("serialization error: {0}")]
    SerializationError(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// `svn_cache__t`
pub struct SvnCache<K, V>
where
    K: Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// storage
    storage: Arc<RwLock<dyn CacheStorage<K, V> + Send + Sync>>,

    /// error handler
    error_handler: Option<Arc<dyn Fn(&CacheError) + Send + Sync>>,

    /// cache statistics
    stats: CacheStats,

    /// whether to pretend the cache is empty
    pretend_empty: bool,
}

impl<K, V> Debug for SvnCache<K, V>
where
    K: Clone + Send + Sync + 'static + Debug,
    V: Clone + Send + Sync + 'static + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SvnCache")
            .field("stats", &self.stats)
            .field("pretend_empty", &self.pretend_empty)
            .finish()
    }
}

pub trait CacheStorage<K, V>
where
    K: Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    fn get(&self, key: &K) -> Result<Option<V>, CacheError>;
    fn has_key(&self, key: &K) -> Result<bool, CacheError>;
    fn set(&mut self, key: K, value: V) -> Result<(), CacheError>;
    fn is_cachable(&self, size: usize) -> bool;
    fn get_info(&self) -> CacheInfo;
    fn clear(&mut self);
}

#[derive(Debug, Clone)]
pub struct CacheInfo {
    pub item_count: u64,
    pub total_size: usize,
}

#[derive(Default, Debug)]
pub struct CacheStats {
    reads: AtomicU64,
    writes: AtomicU64,
    hits: AtomicU64,
    failures: AtomicU64,
}

impl<K, V> SvnCache<K, V>
where
    K: Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new<T>(storage: T) -> Self
    where
        T: CacheStorage<K, V> + Send + Sync + 'static,
    {
        Self {
            storage: Arc::new(RwLock::new(storage)),
            error_handler: None,
            stats: CacheStats::default(),
            pretend_empty: false,
        }
    }

    pub fn get(&self, key: &K) -> Result<Option<V>, CacheError> {
        self.stats.reads.fetch_add(1, Ordering::Relaxed);

        if self.pretend_empty {
            return Ok(None);
        }

        let storage = self.storage.read().unwrap();
        match storage.get(key) {
            Ok(Some(value)) => {
                self.stats.hits.fetch_add(1, Ordering::Relaxed);
                Ok(Some(value))
            }
            Ok(None) => Ok(None),
            Err(e) => {
                self.stats.failures.fetch_add(1, Ordering::Relaxed);
                if let Some(handler) = &self.error_handler {
                    handler(&e);
                }
                Err(e)
            }
        }
    }

    pub fn set(&self, key: K, value: V) -> Result<(), CacheError> {
        self.stats.writes.fetch_add(1, Ordering::Relaxed);

        let mut storage = self.storage.write().unwrap();
        storage.set(key, value).map_err(|e| {
            self.stats.failures.fetch_add(1, Ordering::Relaxed);
            if let Some(handler) = &self.error_handler {
                handler(&e);
            }
            e
        })
    }

    pub fn stats(&self) -> (u64, u64, u64, u64) {
        (
            self.stats.reads.load(Ordering::Relaxed),
            self.stats.writes.load(Ordering::Relaxed),
            self.stats.hits.load(Ordering::Relaxed),
            self.stats.failures.load(Ordering::Relaxed),
        )
    }
}
