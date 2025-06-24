/// The various types of nodes in the Subversion filesystem.
///
/// `svn_node_kind_t`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeKind {
    /// absent
    None,
    /// regular file
    File,
    /// directory
    Directory,
    /// symbolic link
    ///
    /// @note This value is not currently used by the public API.
    Symlink,
    /// something's here, but we don't know what
    Unknown,
}

impl NodeKind {
    /// `kind_map_none`
    pub fn as_str(&self) -> &str {
        match self {
            NodeKind::None => "none",
            NodeKind::File => "file",
            NodeKind::Directory => "dir",
            NodeKind::Symlink => "symlink",
            NodeKind::Unknown => "unknown",
        }
    }
}
