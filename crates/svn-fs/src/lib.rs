mod config;
pub use config::FsConfig;
pub use config::FsFsConfig;
pub use config::FsType;

mod error;
pub use error::Error;

mod fs;
pub use fs::SvnFs;

mod backend;
mod node;
mod root;

pub use node::NodeRevision;

use std::collections::HashMap;

use svn_types::RevisionNumber;
use uuid::Uuid;

/// `fs_vtable_t`
pub trait FsTrait {
    fn youngest_rev(&self) -> RevisionNumber;
    fn refresh_revision_prop(&self) -> Result<(), ()>;

    fn revision_prop(&self) -> Result<(), ()>;
}

/// `svn_fs_access_t`
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None,
    Zlib(i32), // level
    Lz4,
}
