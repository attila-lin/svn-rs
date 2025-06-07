/// The various types of nodes in the Subversion filesystem.
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
