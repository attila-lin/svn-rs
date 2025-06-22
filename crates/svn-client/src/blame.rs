//! blame.c:  return blame messages

use std::{collections::HashMap, path::PathBuf};

use svn_diff::DiffFileOptions;
use svn_subr::opt::OptRevision;
use svn_types::RevisionNumber;

/// The metadata associated with a particular revision.
/// `rev`
pub struct Rev {
    /// The revision number.
    revision: RevisionNumber,
    /// The revison properties.
    rev_props: HashMap<String, String>,
    /// The absolute repository path.
    /// used for merge reporting.
    path: PathBuf,
}

/// One chunk of blame
/// `blame`
pub struct Blame {
    /// the responsible revision
    rev: Rev,
    /// the starting diff-token (line)
    start: usize,
}

/// A chain of blame chunks
pub struct BlameChain {
    /// linked list of blame chunks
    blames: Vec<Blame>,
    /// linked list of free blame chunks
    avail: Vec<Blame>,
}

/// * Invoke @a receiver with @a receiver_baton on each line-blame item
///  * associated with revision @a end of @a path_or_url, using @a start
///  * as the default source of all blame.  @a peg_revision indicates in
///  * which revision @a path_or_url is valid.  If @a peg_revision->kind
///  * is #svn_opt_revision_unspecified, then it defaults to
///  * #svn_opt_revision_head for URLs or #svn_opt_revision_working for
///  * WC targets.
///  *
///  * If @a start->kind or @a end->kind is #svn_opt_revision_unspecified,
///  * return the error #SVN_ERR_CLIENT_BAD_REVISION.  If either are
///  * #svn_opt_revision_working, return the error
///  * #SVN_ERR_UNSUPPORTED_FEATURE.  If any of the revisions of @a
///  * path_or_url have a binary mime-type, return the error
///  * #SVN_ERR_CLIENT_IS_BINARY_FILE, unless @a ignore_mime_type is TRUE,
///  * in which case blame information will be generated regardless of the
///  * MIME types of the revisions.
///  *
///  * @a start may resolve to a revision number greater (younger) than @a end
///  * only if the server is 1.8.0 or greater (supports
///  * #SVN_RA_CAPABILITY_GET_FILE_REVS_REVERSE) and the client is 1.9.0 or
///  * newer.
///  *
///  * Before the first call to @a receiver, set @a *start_revnum_p and
///  * @a *end_revnum_p to the start and end revision number of the entire
///  * blame operation, as resolved from the repository. This can be useful
///  * for the blame receiver to format the blame output. Any or both of these
///  * arguments may be @c NULL.
///  *
///  * Use @a diff_options to determine how to compare different revisions of the
///  * target.
///  *
///  * If @a include_merged_revisions is TRUE, also return data based upon
///  * revisions which have been merged to @a path_or_url.
///  *
///  * Use @a pool for any temporary allocation.
///  *
///  * @since New in 1.12.
/// `svn_client_blame6`
pub fn blame(
    target: &str,
    peg_revision: OptRevision,
    start: OptRevision,
    end: OptRevision,
    diff_options: DiffFileOptions,
) -> Result<(RevisionNumber, RevisionNumber), crate::Error> {
    todo!()
}
