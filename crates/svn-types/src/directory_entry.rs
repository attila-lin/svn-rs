use crate::NodeKind;

/// A general subversion directory entry.
///
/// @note To allow for extending the #svn_dirent_t structure in future
/// releases, always use svn_dirent_create() to allocate the structure.
///
/// @from svn_types.h `svn_dirent_t`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    /// node kind
    kind: NodeKind,
    /// length of file text, otherwise SVN_INVALID_FILESIZE
    size: i64,
    /// does the node have props?
    has_props: bool,
    /// last rev in which this node changed
    created_rev: i64,
    /// time of created_rev (mod-time)
    ///
    /// timestamp
    time: i64,
    /// author of created_rev
    last_author: String,
}

impl Default for DirectoryEntry {
    // `svn_dirent_create`
    fn default() -> Self {
        DirectoryEntry {
            kind: NodeKind::Unknown,
            size: -1,
            has_props: false,
            created_rev: -1,
            time: 0,
            last_author: String::new(),
        }
    }
}
