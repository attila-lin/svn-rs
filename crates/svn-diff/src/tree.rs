use std::path::PathBuf;

use svn_types::RevisionNumber;

/// Describes the source of a merge
///
/// `svn_diff_source_t`
pub struct DiffSource {
    /// Always available
    /// In case of copyfrom: the revision copied from
    revision: RevisionNumber,

    /// In case of copyfrom: the repository relative path copied from.
    ///
    /// NULL if the node wasn't copied or moved, or when the driver doesn't
    /// have this information
    repos_relpath: Option<PathBuf>,

    /// In case of copyfrom: the relative path of source location before the
    /// move. This path is relative WITHIN THE DIFF. The repository path is
    /// typically in repos_relpath
    ///
    /// NULL if the node wasn't moved or if the driver doesn't have this
    /// information.
    moved_from_relpath: Option<PathBuf>,
}
