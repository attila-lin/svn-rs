//!
use std::collections::HashMap;
use std::path::PathBuf;

use svn_types::MergeRange;
use svn_types::NodeKind;
use svn_types::RevisionNumber;
use svn_types::SvnLock;

/// Structure used in the #svn_wc_notify_func2_t function.
///
/// @c kind, @c content_state, @c prop_state and @c lock_state are from
/// after @c action, not before.
///
/// @note If @c action is #svn_wc_notify_update_completed, then @c path has
/// already been installed, so it is legitimate for an implementation of
/// #svn_wc_notify_func2_t to examine @c path in the working copy.
///
/// @note The purpose of the @c kind, @c mime_type, @c content_state, and
/// @c prop_state fields is to provide "for free" information that an
/// implementation is likely to want, and which it would otherwise be
/// forced to deduce via expensive operations such as reading entries
/// and properties.  However, if the caller does not have this
/// information, it will simply pass the corresponding `*_unknown'
/// values, and it is up to the implementation how to handle that
/// (i.e., whether to attempt deduction, or just to punt and
/// give a less informative notification).
///
/// @note Callers of notification functions should use svn_wc_create_notify()
/// or svn_wc_create_notify_url() to create structures of this type to allow
/// for extensibility.
///
/// @since New in 1.2.
///
/// `svn_wc_notify_t`
pub struct Notify {
    /// Path, either absolute or relative to the current working directory
    /// (i.e., not relative to an anchor).  @c path is "." or another valid path
    /// value for compatibility reasons when the real target is a url that
    /// is available in @c url.
    path: PathBuf,
    /// Action that describes what happened to #svn_wc_notify_t.path.
    action: NotifyAction,
    /// Node kind of @c path.
    kind: NodeKind,
    /// If non-NULL, indicates the mime-type of @c path.
    /// It is always @c NULL for directories.
    mime_type: Option<String>,

    /// Points to the lock structure received from the repository when
    /// @c action is #svn_wc_notify_locked.  For other actions, it is
    /// @c NULL.
    lock: Option<SvnLock>,

    /// The type of notification that is occurring about node content.
    content_state: NotifyState,
    /// The type of notification that is occurring about node properties.
    prop_state: NotifyState,
    /// Reflects the addition or removal of a lock token in the working copy.
    lock_state: NotifyState,
    /// When @c action is #svn_wc_notify_update_completed, target revision
    /// of the update, or #SVN_INVALID_REVNUM if not available; when @c
    /// action is #svn_wc_notify_blame_revision, processed revision; Since
    /// Subversion 1.7 when action is #svn_wc_notify_update_update or
    /// #svn_wc_notify_update_add, the target revision.
    /// In all other cases, it is #SVN_INVALID_REVNUM.
    revision: RevisionNumber,

    /// If @c action pertains to a changelist, this is the changelist name.
    /// In all other cases, it is @c NULL.  @since New in 1.5
    changelist_name: String,
    /// When @c action is #svn_wc_notify_merge_begin or
    /// #svn_wc_notify_foreign_merge_begin or
    /// #svn_wc_notify_merge_record_info_begin, and both the
    /// left and right sides of the merge are from the same URL.  In all
    /// other cases, it is @c NULL.  @since New in 1.5
    merge_range: Option<MergeRange>,

    /// Similar to @c path, but if non-NULL the notification is about a url.
    url: String,
    /// If non-NULL, specifies an absolute path prefix that can be subtracted
    /// from the start of the absolute path in @c path or @c url.  Its purpose
    /// is to allow notification to remove a common prefix from all the paths
    /// displayed for an operation.  @since New in 1.6
    path_prefix: Option<PathBuf>,

    /// If @c action relates to properties, specifies the name of the property.
    /// @since New in 1.6
    prop_name: String,

    /// If @c action is #svn_wc_notify_blame_revision, contains a list of
    /// revision properties for the specified revision
    /// @since New in 1.6
    rev_props: HashMap<String, String>,

    /// If @c action is #svn_wc_notify_update_update or
    /// #svn_wc_notify_update_add, contains the revision before the update.
    /// In all other cases, it is #SVN_INVALID_REVNUM.
    /// @since New in 1.7
    old_revision: RevisionNumber,

    /// These fields are used by svn patch to identify the
    /// hunk the notification is for. They are line-based
    /// offsets and lengths parsed from the unidiff hunk header.
    /// @since New in 1.7.
    hunk_original_start: u64,
    hunk_original_length: u64,
    hunk_modified_start: u64,
    hunk_modified_length: u64,

    /// The line at which a hunk was matched (and applied).
    /// @since New in 1.7.
    hunk_matched_line: u64,

    /// The fuzz factor the hunk was applied with.
    /// @since New in 1.7
    hunk_fuzz: u64,
}

/// The type of action occurring.
///
/// `svn_wc_notify_action_t`
pub enum NotifyAction {
    /// Adding a path to revision control.
    Add = 0,

    /// Copying a versioned path.
    Copy,

    /// Deleting a versioned path.
    Delete,

    /// Restoring a missing path from the pristine text-base.
    Restore,

    /// Reverting a modified path.
    /// **Note:** See also svn_wc_notify_revert_noaccess
    Revert,

    /// A revert operation has failed.
    FailedRevert,

    /// All conflicts on a path were marked as resolved.
    /// @note As of 1.10, separate notifications are sent for individually
    /// resolved text, property, and tree conflicts. This notification is used
    /// only if all conflicts on a path were marked resolved at once.
    Resolved,

    /// Skipping a path.
    NotifySkip,

    /// Got a delete in an update.
    UpdateDelete,

    /// Got an add in an update.
    UpdateAdd,

    /// Got any other action in an update.
    UpdateUpdate,

    /// The last notification in an update (including updates of externals).
    UpdateCompleted,

    /// Updating an external module.
    UpdateExternal,

    /// The last notification in a status (including status on externals).
    StatusCompleted,

    /// Running status on an external module.
    StatusExternal,

    /// Committing a modification.
    CommitModified,

    /// Committing an addition.
    CommitAdded,

    /// Committing a deletion.
    CommitDeleted,

    /// Committing a replacement.
    CommitReplaced,

    /// Transmitting post-fix text-delta data for a file.
    CommitPostfixTxdelta,

    /// Processed a single revision's blame.
    BlameRevision,

    /// Locking a path. @since New in 1.2.
    Locked,

    /// Unlocking a path. @since New in 1.2.
    Unlocked,

    /// Failed to lock a path. @since New in 1.2.
    FailedLock,

    /// Failed to unlock a path. @since New in 1.2.
    FailedUnlock,

    /// Tried adding a path that already exists. @since New in 1.5.
    Exists,

    /// Changelist name set. @since New in 1.5.
    ChangelistSet,

    /// Changelist name cleared. @since New in 1.5.
    ChangelistClear,

    /// Warn user that a path has moved from one changelist to another.
    /// @since New in 1.5.
    /// @deprecated As of 1.7, separate clear and set notifications are sent.
    #[deprecated(note = "Use ChangelistSet or ChangelistClear instead")]
    ChangelistMoved,

    /// A merge operation (to path) has begun.  See #svn_wc_notify_t.merge_range.
    /// @since New in 1.5.
    MergeBegin,

    /// A merge operation (to path) from a foreign repository has begun.
    /// See #svn_wc_notify_t.merge_range.  @since New in 1.5.
    ForeignMergeBegin,

    /// Replace notification. @since New in 1.5.
    UpdateReplace,

    /// Property added. @since New in 1.6.
    PropertyAdded,

    /// Property updated. @since New in 1.6.
    PropertyModified,

    /// Property deleted. @since New in 1.6.
    PropertyDeleted,

    /// Nonexistent property deleted. @since New in 1.6.
    PropertyDeletedNonexistent,

    /// Revprop set. @since New in 1.6.
    RevpropSet,

    /// Revprop deleted. @since New in 1.6.
    RevpropDeleted,

    /// The last notification in a merge. @since New in 1.6.
    MergeCompleted,

    /// The path is a tree-conflict victim of the intended action (*not*
    /// a persistent tree-conflict from an earlier operation, but *this*
    /// operation caused the tree-conflict). @since New in 1.6.
    TreeConflict,

    /// The path is a subdirectory referenced in an externals definition
    /// which is unable to be operated on.  @since New in 1.6.
    FailedExternal,

    /// Starting an update operation.  @since New in 1.7.
    UpdateStarted,

    /// An update tried to add a file or directory at a path where
    /// a separate working copy was found.  @since New in 1.7.
    UpdateSkipObstruction,

    /// An explicit update tried to update a file or directory that
    /// doesn't live in the repository and can't be brought in.
    /// @since New in 1.7.
    UpdateSkipWorkingOnly,

    /// An update tried to update a file or directory to which access could
    /// not be obtained. @since New in 1.7.
    UpdateSkipAccessDenied,

    /// An update operation removed an external working copy.
    /// @since New in 1.7.
    UpdateExternalRemoved,

    /// A node below an existing node was added during update.
    /// @since New in 1.7.
    UpdateShadowedAdd,

    /// A node below an existing node was updated during update.
    /// @since New in 1.7.
    UpdateShadowedUpdate,

    /// A node below an existing node was deleted during update.
    /// @since New in 1.7.
    UpdateShadowedDelete,

    /// The mergeinfo on path was updated.  @since New in 1.7.
    MergeRecordInfo,

    /// A working copy directory was upgraded to the latest format.
    /// @since New in 1.7.
    UpgradedPath,

    /// Mergeinfo describing a merge was recorded.
    /// @since New in 1.7.
    MergeRecordInfoBegin,

    /// Mergeinfo was removed due to elision.
    /// @since New in 1.7.
    MergeElideInfo,

    /// A file in the working copy was patched.
    /// @since New in 1.7.
    Patch,

    /// A hunk from a patch was applied.
    /// @since New in 1.7.
    PatchAppliedHunk,

    /// A hunk from a patch was rejected.
    /// @since New in 1.7.
    PatchRejectedHunk,

    /// A hunk from a patch was found to already be applied.
    /// @since New in 1.7.
    PatchHunkAlreadyApplied,

    /// Committing a non-overwriting copy (path is the target of the
    /// copy, not the source).
    /// @since New in 1.7.
    CommitCopied,

    /// Committing an overwriting (replace) copy (path is the target of
    /// the copy, not the source).
    /// @since New in 1.7.
    CommitCopiedReplaced,

    /// The server has instructed the client to follow a URL
    /// redirection.
    /// @since New in 1.7.
    UrlRedirect,

    /// The operation was attempted on a path which doesn't exist.
    /// @since New in 1.7.
    PathNonexistent,

    /// Removing a path by excluding it.
    /// @since New in 1.7.
    Exclude,

    /// Operation failed because the node remains in conflict
    /// @since New in 1.7.
    FailedConflict,

    /// Operation failed because an added node is missing
    /// @since New in 1.7.
    FailedMissing,

    /// Operation failed because a node is out of date
    /// @since New in 1.7.
    FailedOutOfDate,

    /// Operation failed because an added parent is not selected
    /// @since New in 1.7.
    FailedNoParent,

    /// Operation failed because a node is locked by another user and/or
    /// working copy.  @since New in 1.7.
    FailedLocked,

    /// Operation failed because the operation was forbidden by the server.
    /// @since New in 1.7.
    FailedForbiddenByServer,

    /// The operation skipped the path because it was conflicted.
    /// @since New in 1.7.
    SkipConflicted,

    /// Just the lock on a file was removed during update.
    /// @since New in 1.8.
    UpdateBrokenLock,

    /// Operation failed because a node is obstructed.
    /// @since New in 1.8.
    FailedObstruction,

    /// Conflict resolver is starting.
    /// This can be used by clients to detect when to display conflict summary
    /// information, for example.
    /// @since New in 1.8.
    ConflictResolverStarting,

    /// Conflict resolver is done.
    /// This can be used by clients to detect when to display conflict summary
    /// information, for example.
    /// @since New in 1.8.
    ConflictResolverDone,

    /// The current operation left local changes of something that was deleted
    /// The changes are available on (and below) the notified path
    /// @since New in 1.8.
    LeftLocalModifications,

    /// A copy from a foreign repository has started
    /// @since New in 1.8.
    ForeignCopyBegin,

    /// A move in the working copy has been broken, i.e. degraded into a
    /// copy + delete. The notified path is the move source (the deleted path).
    /// ### TODO: Provide path to move destination as well?
    /// @since New in 1.8.
    MoveBroken,

    /// Running cleanup on an external module.
    /// @since New in 1.9.
    CleanupExternal,

    /// The operation failed because the operation (E.g. commit) is only valid
    /// if the operation includes this path.
    /// @since New in 1.9.
    FailedRequiresTarget,

    /// Running info on an external module.
    /// @since New in 1.9.
    InfoExternal,

    /// Finalizing commit.
    /// @since New in 1.9.
    CommitFinalizing,

    /// All text conflicts in a file were marked as resolved.
    /// @since New in 1.10.
    ResolvedText,

    /// A property conflict on a path was marked as resolved.
    /// The name of the property is specified in #svn_wc_notify_t.prop_name.
    /// @since New in 1.10.
    ResolvedProp,

    /// A tree conflict on a path was marked as resolved.
    /// @since New in 1.10.
    ResolvedTree,

    /// Starting to search the repository for details about a tree conflict.
    /// @since New in 1.10.
    BeginSearchTreeConflictDetails,

    /// Progressing in search of repository for details about a tree conflict.
    /// The revision being searched is specified in #svn_wc_notify_t.revision.
    /// @since New in 1.10.
    TreeConflictDetailsProgress,

    /// Done searching the repository for details about a conflict.
    /// @since New in 1.10.
    EndSearchTreeConflictDetails,

    /// Hydrating (fetching text-bases): starting a batch of fetching
    /// within the WC subtree at @c svn_wc_notify_t.path. (Zero or more files
    /// may be fetched, each preceded by @c svn_wc_notify_hydrating_file.)
    /// @since New in 1.15.
    HydratingStart,

    /// Hydrating (fetching text-bases): about to fetch a file
    /// from @c svn_wc_notify_t.url at @c svn_wc_notify_t.revision.
    /// @since New in 1.15.
    HydratingFile,

    /// Hydrating (fetching text-bases): finished a batch of fetching
    /// within the WC subtree at @c svn_wc_notify_t.path.
    /// @since New in 1.15.
    HydratingEnd,

    /// A warning, specified in #svn_wc_notify_t.err.
    /// @since New in 1.15.
    Warning,

    /// A file is readonly for the user but isn't svn:needs-lock.
    /// So we want to restore RW, but fail since the file has W bits,
    /// just not for the current user.
    /// @since New in 1.15.
    Noaccess,
}

/// The type of notification that is occurring.
///
/// `svn_wc_notify_state_t`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifyState {
    Inapplicable = 0,

    /// Notifier doesn't know or isn't saying.
    Unknown,

    /// The state did not change.
    Unchanged,

    /// The item wasn't present.
    Missing,

    /// An unversioned item obstructed work.
    Obstructed,

    /// Pristine state was modified.
    Changed,

    /// Modified state had mods merged in.
    Merged,

    /// Modified state got conflicting mods.
    Conflicted,

    /// The source to copy the file from is missing.
    SourceMissing,
}

/// Notify the world that @a notify->action has happened to @a notify->path.
///
/// Recommendation: callers of #svn_wc_notify_func2_t should avoid
/// invoking it multiple times on the same path within a given
/// operation, and implementations should not bother checking for such
/// duplicate calls.  For example, in an update, the caller should not
/// invoke the notify func on receiving a prop change and then again
/// on receiving a text change.  Instead, wait until all changes have
/// been received, and then invoke the notify func once (from within
/// an #svn_delta_editor_t's close_file(), for example), passing
/// the appropriate @a notify->content_state and @a notify->prop_state flags.
///
/// @since New in 1.2.
///
/// `svn_wc_notify_func2_t`
pub type NotifyFunc = Box<dyn Fn() -> ()>;
