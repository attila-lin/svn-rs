use std::collections::HashMap;

use crate::FsAccess;

/// An object representing a Subversion filesystem.
///
/// `svn_fs_t`
pub struct SvnFs {
    ///  The path to the repository's top-level directory
    path: String,

    /// The filesystem configuration
    config: HashMap<(), ()>,

    /// An access context indicating who's using the fs
    access_ctx: FsAccess,
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
