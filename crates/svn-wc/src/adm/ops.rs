use std::collections::HashMap;

/// `svn_wc_committed_queue_t`
pub struct CommittedQueue {
    /* Mapping (const char *) wcroot_abspath to svn_wc__db_commit_queue_t * */
    wc_queues: HashMap<String, CommitedQueueItem>,
}

/// `commited_queue_item_t`
pub struct CommitedQueueItem {
    /// Mapping (const char *) wcroot_abspath to svn_wc__db_commit_queue_t *
    local_abspath: String,
    /// Use legacy recursion
    recurse: bool,
    /// Process the node as committed
    committed: bool,
    /// Remove existing lock on node.
    remove_lock: bool,
    /// Remove changelist on node.
    remove_changelist: bool,
}
