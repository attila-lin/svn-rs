/// Enumerated values describing the state of a node.
///
/// `svn_wc__db_status_t`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbStatus {
    /// The node is present and has no known modifications applied to it.
    Normal,
    ///  The node has been added (potentially obscuring a delete or move of
    //        the BASE node; see HAVE_BASE param [### What param? This is an enum
    //        not a function.] ). The text will be marked as
    //        modified, and if properties exist, they will be marked as modified.
    //
    //        In many cases svn_wc__db_status_added means any of added, moved-here
    //        or copied-here. See individual functions for clarification and
    //        svn_wc__db_scan_addition() to get more details.
    Added,
    /// This node has been added with history, based on the move source.
    //        Text and property modifications are based on whether changes have
    //        been made against their pristine versions.
    MovedHere,

    /// This node has been added with history, based on the copy source.
    //        Text and property modifications are based on whether changes have
    //        been made against their pristine versions.
    Copied,

    /// This node has been deleted. No text or property modifications
    //        will be present.
    Deleted,
    /// This node was named by the server, but no information was provided.
    ServerExcluded,

    /// This node has been administratively excluded.
    Excluded,

    /// This node is not present in this revision. This typically happens
    //        when a node is deleted and committed without updating its parent.
    //        The parent revision indicates it should be present, but this node's
    //        revision states otherwise.
    NotPresent,

    /// This node is known, but its information is incomplete. Generally,
    //        it should be treated similar to the other missing status values
    //        until some (later) process updates the node with its data.
    //
    //        When the incomplete status applies to a directory, the list of
    //        children and the list of its base properties as recorded in the
    //        working copy do not match their working copy versions.
    //        The update editor can complete a directory by using a different
    //        update algorithm.
    Incomplete,

    /// The BASE node has been marked as deleted. Only used as an internal
    //        status in wc_db.c and entries.c.
    BaseDeleted,
}

/// Lock information.  We write/read it all as one, so let's use a struct
///   for convenience.
///
/// `svn_wc__db_lock_t`
#[derive(Debug, Clone)]
pub struct DbLock {
    /// The token of the lock.
    pub token: String,
    /// The owner of the lock, possibly NULL.
    pub owner: Option<String>,
    /// A comment about the lock, possibly NULL
    pub comment: Option<String>,
    /// The date the lock was created
    pub date: i64,
}

/// Context data structure for interacting with the administrative data.
///
/// `svn_wc__db_t`
#[derive(Debug)]
pub struct WcDb {}
