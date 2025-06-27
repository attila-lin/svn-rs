use crate::RevisionNumber;

/// All information about a commit.
///
/// @note Objects of this type should always be created using the
///  [`svn_create_commit_info`]() function.
///
/// @from svn_types.h `svn_commit_info_t`
pub struct CommitInfo {
    /// just-committed revision.
    revision: RevisionNumber,
    /// server-side date of the commit.
    date: String,
    /// author of the commit.
    author: String,
    /// error message from post-commit hook, or NULL.
    post_commit_err: Option<String>,
    /// repository root, may be @c NULL if unknown.
    repos_root: Option<String>,
}
