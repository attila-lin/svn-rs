use svn_types::NodeKind;

/// Node-Revision
///
/// `node_revision_t`
pub struct NodeRevision {
    /// node kind
    pub kind: NodeKind,
    // /// The node-id for this node-rev.
    // pub id: TODO
    /// predecessor node revision id, or NULL if there is no predecessor
    /// for this node revision
    pub predecessor_id: Option<String>,

    /// If this node-rev is a copy, where was it copied from?
    ///
    /// FIXME: make it `CopyFromInfo` struct together with `copyfrom_path` and `copyfrom_rev`
    copyfrom_path: Option<String>,
    copyfrom_rev: Option<i64>,
}
