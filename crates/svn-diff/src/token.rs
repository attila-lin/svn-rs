//! routines for doing diffs
//!

/// Prime number to use as the size of the hash table.  This number was
/// not selected by testing of any kind and may need tweaking.
const SVN_DIFF__HASH_SIZE: usize = 127;

/// `svn_diff__node_t`
pub struct SvnDiffNode {
    parent: Arc<SvnDiffNode>,
    left: Arc<SvnDiffNode>,
    right: Arc<SvnDiffNode>,

    hash: u32,
    index: i64,
    // FIXME: token type
}

/// `svn_diff__tree_t`
pub struct SvnDiffTree {
    root: [SvnDiffNode; SVN_DIFF__HASH_SIZE],
    pub node_count: i64
}
