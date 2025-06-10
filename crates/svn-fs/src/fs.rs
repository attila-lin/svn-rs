use std::collections::HashMap;
use std::path::{Path, PathBuf};

use uuid::Uuid;

use crate::FsAccess;
use crate::fsfs::FsFsData;

/// An object representing a Subversion filesystem.
///
/// `svn_fs_t`
pub struct SvnFs {
    ///  The path to the repository's top-level directory
    pub path: PathBuf,

    /// The filesystem configuration
    config: HashMap<(), ()>,

    /// An access context indicating who's using the fs
    access_ctx: FsAccess,

    /// FSAP-specific vtable and private data
    ///
    /// FIXME: now use [`FsFsData`]
    data: FsFsData,

    /// UUID, stored by open(), create(), and set_uuid().
    uuid: Uuid,
}

impl super::FsTrait for SvnFs {
    fn youngest_rev(&self) -> super::RevisionNumber {
        todo!()
    }

    fn refresh_revision_prop(&self) -> Result<(), ()> {
        todo!()
    }

    fn revision_prop(&self) -> Result<(), ()> {
        todo!()
    }
}
