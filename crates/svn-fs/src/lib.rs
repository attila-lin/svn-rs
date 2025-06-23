mod config;
pub mod txn;
pub use config::FsConfig;
pub use config::FsFsConfig;
pub use config::FsType;

mod error;
pub use error::Error;

mod fs;
pub use fs::SvnFs;

pub mod backend;
mod node;

mod root;
pub use root::FsRoot;

pub use node::NodeRevision;

mod util;

use std::collections::HashMap;
use std::fmt::Debug;

use uuid::Uuid;

/// `svn_fs_access_t`
#[derive(Debug, PartialEq, Eq)]
pub struct FsAccess {
    /// An authenticated username using the fs
    pub username: String,
    /// A collection of lock-tokens supplied by the fs caller.
    ///
    /// Hash maps (const char *) UUID --> path where path can be the
    /// magic value (void *) 1 if no path was specified.
    /// fs functions should really only be interested whether a UUID
    /// exists as a hash key at all;  the value is irrelevant.
    pub lock_tokens: HashMap<Uuid, String>,
}

/// `compression_type_t`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompressionType {
    #[default]
    None,
    Zlib(i32), // level
    Lz4,
}
