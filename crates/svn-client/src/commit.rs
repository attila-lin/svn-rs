// pub fn reconcile_er

// pub collect_lock_tokens()
//

mod util;

use std::path::PathBuf;

use svn_types::{NodeKind, RevisionNumber};
use url::Url;

fn post_process_commit_item() -> () {}

/// The commit candidate structure.
///
/// In order to avoid backwards compatibility problems clients should use
/// svn_client_commit_item3_create() to allocate and initialize this
/// structure instead of doing so themselves.
///
/// @since New in 1.5.
///
/// `svn_client_commit_item_t`
#[derive(Debug, Clone, Default)]
pub struct CommitItem {
    /// absolute working-copy path of item. Always set during normal commits
    /// (and copies from a working copy) to the repository. Can only be NULL
    /// when stub commit items are created for operations that only involve
    /// direct repository operations. During WC->REPOS copy operations, this
    /// path is the WC source path of the operation.
    path: PathBuf,
    /// node kind (dir, file)
    kind: NodeKind,

    /// commit URL for this item. Points to the repository location of PATH
    /// during commits, or to the final URL of the item when copying from the
    /// working copy to the repository.
    url: Option<Url>,
    /// revision of textbase
    revision: Option<RevisionNumber>,
    /// copyfrom-url or NULL if not a copied item
    copyfrom_url: Option<Url>,
    /// copyfrom-revision or None if not a copied item
    copyfrom_revision: Option<RevisionNumber>,
    /// state flags
    state_flags: CommitItemStateFlags,

    /** An array of #svn_prop_t *'s, which are incoming changes from
     * the repository to WC properties.  These changes are applied
     * post-commit.
     *
     * When adding to this array, allocate the #svn_prop_t and its
     * contents in @c incoming_prop_changes->pool, so that it has the
     * same lifetime as this data structure.
     *
     * See https://issues.apache.org/jira/browse/SVN-806 for a
     * description of what would happen if the post-commit process
     * didn't group these changes together with all other changes to the
     * item.
     */
    imcoming_prop_changes: Vec<Prop>,
}
