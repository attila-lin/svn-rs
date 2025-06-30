use svn_types::SVN_INVALID_REVNUM;
use svn_types::SvnLock;
use svn_types::{Depth, NodeKind, RevisionNumber};
use uuid::Uuid;

/// The type of status for the working copy.
/// `svn_wc_status_kind`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusKind {
    /// does not exist
    None = 1,
    /// is not a versioned thing in this wc
    Unversioned,
    /// exists, but uninteresting
    Normal,
    /// is scheduled for addition
    Added,
    /// under v.c., but is missing
    Missing,
    /// scheduled for deletion
    Deleted,
    /// was deleted and then re-added
    Replaced,
    /// text or props have been modified
    Modified,
    /// local mods received repos mods (### unused)
    Merged,
    /// local mods received conflicting repos mods
    Conflicted,
    /// is unversioned but configured to be ignored
    Ignored,
    /// an unversioned resource is in the way of the versioned resource
    Obstructed,
    /// an unversioned directory path populated by an svn:externals
    /// property; this status is not used for file externals
    External,
    /// a directory doesn't contain a complete entries list
    Incomplete,
}

/// Structure for holding the "status" of a working copy item.
///
/// @note Fields may be added to the end of this structure in future
/// versions.  Therefore, to preserve binary compatibility, users
/// should not directly allocate structures of this type.
///
/// `svn_wc_status3_t`
pub struct Status {
    /// The kind of node as recorded in the working copy
    kind: NodeKind,
    /// The depth of the node as recorded in the working copy
    /// (#svn_depth_unknown for files or when no depth is set)
    depth: Depth,
    /// The actual size of the working file on disk, or SVN_INVALID_FILESIZE
    /// if unknown (or if the item isn't a file at all).
    filesize: i64,
    /// If the path is under version control, versioned is TRUE, otherwise
    /// FALSE.
    pub versioned: bool,
    /// Set to TRUE if the item is the victim of a conflict.
    conflicted: bool,
    /// The status of the node itself. In order of precedence: Obstructions,
    /// structural changes, text changes.
    pub node_status: StatusKind,
    /// The status of the entry's text.
    pub text_status: StatusKind,
    /// The status of the entry's properties.
    pub prop_status: StatusKind,

    /// a file or directory can be 'copied' if it's scheduled for
    /// addition-with-history (or part of a subtree that is scheduled as such.).
    copied: bool,
    /// Base revision.
    revision: RevisionNumber,
    /// Last revision this was changed
    changed_rev: RevisionNumber,
    /// Date of last commit.
    changed_date: i64,
    /// Last commit author of this item
    changed_author: String,
    /// The URL of the repository
    repos_root_url: String,
    /// The UUID of the repository
    repos_uuid: Uuid,
    /// The in-repository path relative to the repository root.
    repo_relpath: String,
    /// a file or directory can be 'switched' if the switch command has been
    /// used.  If this is TRUE, then file_external will be FALSE.
    switched: bool,
    /// This directory has a working copy lock
    locked: bool,
    /// The repository file lock. (Values of path, token, owner, comment
    /// and are available if a lock is present)
    lock: SvnLock,
    /// Which changelist this item is part of, or NULL if not part of any.
    changelist: Option<String>,

    // @defgroup svn_wc_status_ood WC out-of-date info from the repository
    // @{
    //
    // When the working copy item is out-of-date compared to the
    // repository, the following fields represent the state of the
    // youngest revision of the item in the repository.  If the working
    // copy is not out of date, the fields are initialized as described
    // below.
    /// Set to the node kind of the youngest commit, or #svn_node_none
    /// if not out of date.
    ood_kind: NodeKind,

    /// The status of the node, based on the text status if the node has no
    /// restructuring changes
    repos_node_status: StatusKind,

    /// The entry's text status in the repository.
    repos_text_status: StatusKind,

    /// The entry's property status in the repository.
    repos_prop_status: StatusKind,

    /// The entry's lock in the repository, if any.
    repos_lock: Option<SvnLock>,

    /// Set to the youngest committed revision, or #SVN_INVALID_REVNUM
    /// if not out of date.
    ood_changed_rev: RevisionNumber,

    /// Set to the most recent commit date, or @c 0 if not out of date.
    ood_changed_date: i64,
    /// Set to the user name of the youngest commit, or @c NULL if not
    /// out of date or non-existent.  Because a non-existent @c
    /// svn:author property has the same behavior as an out-of-date
    /// working copy, examine @c ood_last_cmt_rev to determine whether
    /// the working copy is out of date.
    ood_changed_author: Option<String>,

    /// Set to the local absolute path that this node was moved from, if this
    /// file or directory has been moved here locally and is the root of that
    /// move. Otherwise set to NULL.
    ///
    /// This will be NULL for moved-here nodes that are just part of a subtree
    /// that was moved along (and are not themselves a root of a different move
    /// operation).
    moved_from_abspath: Option<String>,

    /// Set to the local absolute path that this node was moved to, if this file
    /// or directory has been moved away locally and corresponds to the root
    /// of the destination side of the move. Otherwise set to NULL.
    ///
    /// Note: Saying just "root" here could be misleading. For example:
    ///   svn mv A AA;
    ///   svn mv AA/B BB;
    /// creates a situation where A/B is moved-to BB, but one could argue that
    /// the move source's root actually was AA/B. Note that, as far as the
    /// working copy is concerned, above case is exactly identical to:
    ///   svn mv A/B BB;
    ///   svn mv A AA;
    /// In both situations, @a moved_to_abspath would be set for nodes A (moved
    /// to AA) and A/B (moved to BB), only.
    ///
    /// This will be NULL for moved-away nodes that were just part of a subtree
    /// that was moved along (and are not themselves a root of a different move
    /// operation).
    moved_to_abspath: Option<String>,

    /// @c TRUE iff the item is a file brought in by an svn:externals definition.
    file_external: bool,

    /// The actual kind of the node in the working copy. May differ from
    /// @a kind on obstructions, deletes, etc. #svn_node_unknown if unavailable.
    actual_kind: NodeKind,
}

/// A structure to report a summary of a working copy, including the
/// mix of revisions found within it, whether any parts are switched or
/// locally modified, and whether it is a sparse checkout.
///
/// @note Fields may be added to the end of this structure in future
/// versions.  Therefore, to preserve binary compatibility, users
/// should not directly allocate structures of this type.
///
/// `svn_wc_revision_status_t`
pub struct RevisionStatus {
    /// Lowest revision found
    pub min_rev: RevisionNumber,
    /// Highest revision found
    pub max_rev: RevisionNumber,

    /// Is anything switched?
    pub switched: bool,
    /// Is anything modified?
    pub modified: bool,

    /// Whether any WC paths are at a depth other than #svn_depth_infinity or
    /// are user excluded.
    pub sparse_checkout: bool,
}

impl Default for RevisionStatus {
    fn default() -> Self {
        Self {
            min_rev: SVN_INVALID_REVNUM,
            max_rev: SVN_INVALID_REVNUM,
            switched: false,
            modified: false,
            sparse_checkout: false,
        }
    }
}
