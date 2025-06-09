mod config;
pub use config::FsFsConfig;
pub use config::FsType;

mod fs;
pub use fs::SvnFs;

use std::collections::HashMap;

use uuid::Uuid;

mod fsx;
mod root;

use svn_types::RevisionNumber;

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
    //      Hash maps (const char *) UUID --> path where path can be the
    //      magic value (void *) 1 if no path was specified.
    //      fs functions should really only be interested whether a UUID
    //      exists as a hash key at all;  the value is irrelevant.
    lock_tokens: HashMap<Uuid, String>,
}
