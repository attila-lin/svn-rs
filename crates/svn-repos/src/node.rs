//! * @defgroup svn_repos_inspection Data structures and editor things for \
//! * repository inspection.
//! * @{
//! *
//! * As it turns out, the svn_repos_replay2(), svn_repos_dir_delta2() and
//! * svn_repos_begin_report3() interfaces can be extremely useful for
//! * examining the repository, or more exactly, changes to the repository.
//! * These drivers allows for differences between two trees to be
//! * described using an editor.
//! *
//! * By using the editor obtained from svn_repos_node_editor() with one of
//! * the drivers mentioned above, the description of how to transform one
//! * tree into another can be used to build an in-memory linked-list tree,
//! * which each node representing a repository node that was changed.

/// A node in the repository.
///
/// `svn_repos_node_t`
pub struct Node {
    /// Node type (file, directory, etc.)
    kind: NodeKind,
    /// How this node entered the node tree: 'A'dd, 'D'elete, 'R'eplace
    action: NodeAction,
    /// Were there any textual mods? (files only)
    text_mod: bool,
    /// Where there any property mods?
    prop_mod: bool,
    /// The name of this node as it appears in its parent's entries list
    name: String,
    /// The filesystem revision where this was copied from (if any)
    copyfrom_rev: RevisionNumber,

    /// The filesystem path where this was copied from (if any)
    copyfrom_path: String,
    /// Pointer to the next sibling of this node
    sibling: Option<Arc<Node>>,

    /// Pointer to the first child of this node
    child: Option<Arc<Node>>,

    /// Pointer to the parent of this node
    parent: Option<Arc<Node>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeAction {
    Add,
    Update,
    Replace,
}

impl Node {
    /// Node creation and assembly structures and routines. ***/
    ///
    /// `create_node`
    pub fn new(name: &str, parent: Option<Arc<Node>>) -> Self {
        Node {
            kind: NodeKind::Unknown,
            action: NodeAction::Replace,
            text_mod: false,
            prop_mod: false,
            name: name.to_string(),
            copyfrom_rev: RevisionNumber::default(),
            copyfrom_path: String::new(),
            sibling: None,
            child: None,
            parent: parent.clone(),
        }
    }

    /// `create_sibling_node`
    pub fn new_sibling(elder: Option<Arc<Node>>, name: &str) -> Option<Self> {
        /* No ELDER sibling?  That's just not gonna work out. */
        if elder.is_none() {
            return None;
        }

        let temp_node = elder.unwrap();
        /* Run to the end of the list of siblings of ELDER. */
        while temp_node.sibling.is_some() {
            // Find the last sibling
            if let Some(sibling) = &temp_node.sibling {
                temp_node = sibling.clone();
            } else {
                break;
            }
        }
        /* Create a new youngest sibling and return that. */
        temp_node = sibling = Self::new(name, elder.parent);

        Ok(temp_node)
    }
}
