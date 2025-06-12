/// The various types of nodes in the Subversion filesystem.
///
/// `svn_node_kind_t`
#[derive(Debug)]
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
