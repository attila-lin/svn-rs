//!  DAG-like interface filesystem

use svn_types::NodeKind;

use crate::NodeRevision;
use crate::SvnFs;

/// Generic DAG node stuff.
/// `dag_node_t`
pub struct DagNode {
    /// The filesystem this dag node came from.
    fs: SvnFs,

    // /// The node revision ID for this dag node, allocated in POOL.
    // id:
    /// The node's type (file, dir, etc.)
    kind: NodeKind,

    ///  The node's NODE-REVISION, or NULL if we haven't read it in yet.
    /// This is allocated in this node's POOL.
    ///
    /// If you're willing to respect all the rules above, you can munge
    /// this yourself, but you're probably better off just calling
    /// `get_node_revision` and `set_node_revision`, which take care of
    /// things for you.
    node_revision: NodeRevision,
}
