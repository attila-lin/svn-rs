//! Roots.
//  *
//  * An #svn_fs_root_t object represents the root directory of some
//  * revision or transaction in a filesystem.  To refer to particular
//  * node or node revision, you provide a root, and a directory path
//  * relative to that root.

use std::collections::HashMap;

use crate::fs::SvnFs;

/// The Filesystem Root object.
///
/// `svn_fs_root_t`
pub struct FsRoot {
    /// The filesystem to which this root belongs
    fs: Box<SvnFs>,

    /// The kind of root this is
    is_txn_root: bool,

    ///  For transaction roots, the name of the transaction
    txn: Option<String>,
}

/// `root_vtable_t`
///
/// Some of these operations accept multiple root arguments.  Since the
/// roots may not all have the same vtable, we need a rule to determine
/// which root's vtable is used.  The rule is: if one of the roots is
/// named "target", we use that root's vtable; otherwise, we use the
/// first root argument's vtable.
/// These callbacks correspond to svn_fs_* functions in include/svn_fs.h,
/// see there for details.
///
/// @Note: delete_node() corresponds to svn_fs_delete().
pub trait RootTrait {
    fn paths_changed(&self, changed_paths: &HashMap<(), ()>) -> Result<(), ()>;
}
