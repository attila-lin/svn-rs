//!
use svn_types::SvnLock;
use svn_types::{NodeKind, RevisionNumber};
use svn_wc::info::WcInfo;
use uuid::Uuid;

/// A structure which describes various system-generated metadata about
/// a working-copy path or URL.
/// `svn_client_info2_t`
///
/// @note Fields may be added to the end of this structure in future
/// versions.  Therefore, users shouldn't allocate structures of this
/// type, to preserve binary compatibility.
///
/// @since New in 1.7.
#[derive(Debug, Clone)]
pub struct ClientInfo {
    /// Where the item lives in the repository.
    url: String,

    /** The revision of the object.  If the target is a working-copy
     * path, then this is its current working revnum.  If the target
     * is a URL, then this is the repos revision that it lives in. */
    rev: Option<RevisionNumber>,

    /// The root URL of the repository.
    repos_root_url: String,

    /// The repository's UUID.
    repos_uuid: Uuid,

    /// The node's kind.
    kind: NodeKind,

    /** The size of the file in the repository (untranslated,
     * e.g. without adjustment of line endings and keyword
     * expansion). Only applicable for file -- not directory -- URLs.
     * For working copy paths, @a size will be #SVN_INVALID_FILESIZE. */
    size: u64,
    /// The last revision in which this object changed.
    last_changed_rev: Option<RevisionNumber>,

    /// The date of the last changed revision.
    last_changed_date: Option<i64>,

    /** An exclusive lock, if present.  Could be either local or remote. */
    lock: Option<SvnLock>,
    /** Possible information about the working copy, NULL if not valid. */
    wc_info: WcInfo,
}
