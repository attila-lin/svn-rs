use std::collections::HashMap;
use std::path::{Path, PathBuf};

use svn_subr::SvnConfig;
use svn_types::NodeKind;

use crate::Error;
use crate::status::RevisionStatus;

mod error;
pub mod sql;
pub use error::DBError;

mod wcroot;

use crate::root::WcRoot;

/// Enumerated values describing the state of a node.
///
/// `svn_wc__db_status_t`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbStatus {
    /// The node is present and has no known modifications applied to it.
    Normal,
    /// The node has been added (potentially obscuring a delete or move of
    /// the BASE node; see HAVE_BASE param [### What param? This is an enum
    /// not a function.] ). The text will be marked as
    /// modified, and if properties exist, they will be marked as modified.
    //
    /// In many cases svn_wc__db_status_added means any of added, moved-here
    /// or copied-here. See individual functions for clarification and
    /// `svn_wc__db_scan_addition`() to get more details.
    Added,
    /// This node has been added with history, based on the move source.
    /// Text and property modifications are based on whether changes have
    /// been made against their pristine versions.
    MovedHere,

    /// This node has been added with history, based on the copy source.
    /// Text and property modifications are based on whether changes have
    /// been made against their pristine versions.
    Copied,

    /// This node has been deleted. No text or property modifications
    /// will be present.
    Deleted,
    /// This node was named by the server, but no information was provided.
    ServerExcluded,

    /// This node has been administratively excluded.
    Excluded,

    /// This node is not present in this revision. This typically happens
    /// when a node is deleted and committed without updating its parent.
    /// The parent revision indicates it should be present, but this node's
    /// revision states otherwise.
    NotPresent,

    /// This node is known, but its information is incomplete. Generally,
    /// it should be treated similar to the other missing status values
    /// until some (later) process updates the node with its data.
    ///
    /// When the incomplete status applies to a directory, the list of
    /// children and the list of its base properties as recorded in the
    /// working copy do not match their working copy versions.
    /// The update editor can complete a directory by using a different
    /// update algorithm.
    Incomplete,

    /// The BASE node has been marked as deleted. Only used as an internal
    /// status in wc_db.c and entries.c.
    BaseDeleted,
}

/// Lock information.  We write/read it all as one, so let's use a struct
/// for convenience.
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
pub struct WcDb {
    /// We need the config whenever we run into a new WC directory, in order
    /// to figure out where we should look for the corresponding datastore.
    config: SvnConfig,

    /// Should we fail with SVN_ERR_WC_UPGRADE_REQUIRED when it is
    /// opened, and found to be not-current?
    verify_format: bool,

    /// Should we ensure the WORK_QUEUE is empty when a DB is locked
    /// for writing?
    enforce_empty_work_queue: bool,

    ///  Should we open Sqlite databases EXCLUSIVE
    exclusive: bool,

    /// Busy timeout in ms., 0 for the libsvn_subr default.
    timeout: i32,

    /// Map a given working copy directory to its relevant data.
    /// const char *local_abspath -> svn_wc__db_wcroot_t *wcroot
    dir_data: HashMap<PathBuf, WcRoot>,
    /// A few members to assist with caching of kind values for paths.
    /// See `get_path_kind()` for use.
    parse_cache: ParseCache,
}

#[derive(Debug)]
struct ParseCache {
    abspath: String,
    kind: NodeKind,
}

impl WcDb {
    /// Open a working copy administrative database context.
    ///
    /// This context is (initially) not associated with any particular working
    /// copy directory or working copy root (wcroot). As operations are performed,
    /// this context will load the appropriate wcroot information.
    ///
    /// The context is returned in DB.
    ///
    /// CONFIG should hold the various configuration options that may apply to
    /// the administrative operation. It should live at least as long as the
    /// RESULT_POOL parameter.
    ///
    /// When OPEN_WITHOUT_UPGRADE is TRUE, then the working copy databases will
    /// be opened even when an old database format is found/detected during
    /// the operation of a wc_db API). If open_without_upgrade is FALSE and an
    /// upgrade is required, then SVN_ERR_WC_UPGRADE_REQUIRED will be returned
    /// from that API.
    /// Passing TRUE will allow a bare minimum of APIs to function (most notably,
    /// the temp_get_format() function will always return a value) since most of
    /// these APIs expect a current-format database to be present.
    ///
    /// If ENFORCE_EMPTY_WQ is TRUE, then any databases with stale work items in
    /// their work queue will raise an error when they are opened. The operation
    /// will raise SVN_ERR_WC_CLEANUP_REQUIRED. Passing FALSE for this routine
    /// means that the work queue is being processed (via 'svn cleanup') and all
    /// operations should be allowed.
    ///
    /// The DB will be closed when RESULT_POOL is cleared. It may also be closed
    /// manually using svn_wc__db_close(). In particular, this will close any
    /// SQLite databases that have been opened and cached.
    ///
    /// The context is allocated in RESULT_POOL. This pool is *retained* and used
    /// for future allocations within the DB. Be forewarned about unbounded
    /// memory growth if this DB is used across an unbounded number of wcroots
    /// and versioned directories.
    ///
    /// `svn_wc__db_open`
    pub fn open() -> Result<Self, Error> {
        todo!()
    }

    /// Gather revision status information about a working copy using DB.
    ///
    /// Set *MIN_REVISION and *MAX_REVISION to the lowest and highest revision
    /// numbers found within LOCAL_ABSPATH.
    /// Only nodes with op_depth zero and presence 'normal' or 'incomplete'
    /// are considered, so that added, deleted or excluded nodes do not affect
    /// the result.  If COMMITTED is TRUE, set *MIN_REVISION and *MAX_REVISION
    /// to the lowest and highest committed (i.e. "last changed") revision numbers,
    /// respectively.
    ///
    /// Indicate in *IS_SPARSE_CHECKOUT whether any of the nodes within
    /// LOCAL_ABSPATH is sparse.
    /// Indicate in *IS_MODIFIED whether the working copy has local modifications
    /// recorded for it in DB.
    ///
    /// Indicate in *IS_SWITCHED whether any node beneath LOCAL_ABSPATH
    /// is switched. If TRAIL_URL is non-NULL, use it to determine if LOCAL_ABSPATH
    /// itself is switched.  It should be any trailing portion of LOCAL_ABSPATH's
    /// expected URL, long enough to include any parts that the caller considers
    /// might be changed by a switch.  If it does not match the end of WC_PATH's
    /// actual URL, then report a "switched" status.
    ///
    /// See also the functions below which provide a subset of this functionality.
    ///
    /// `svn_wc__db_revision_status`
    pub fn revision_status(
        &self,
        local_abspath: &Path,
        trail_url: &Option<String>,
        commited: bool,
    ) -> Result<RevisionStatus, Error> {
        if !local_abspath.is_absolute() {
            return Err(Error::NotAbsolutePath(
                local_abspath.to_string_lossy().to_string(),
            ));
        }

        let local_relpath = self.wcroot_parse_local_abspath(local_abspath)?;

        todo!()
    }

    /// Set *MODIFIED to true if there are any local modifications within the
    /// tree rooted at LOCAL_ABSPATH, using DB. If *MODIFIED
    /// is set to true and all the local modifications were deletes then set
    /// *ALL_EDITS_ARE_DELETES to true, set it to false otherwise.
    ///
    /// LOCAL_ABSPATH may be a file or a directory.
    ///
    /// `svn_wc__node_has_local_mods`
    pub fn node_has_local_mods(
        &self,
        local_abspath: &Path,
        ignore_unversioned: bool,
    ) -> Result<LocalModsInfo, Error> {
        // if !all_edits_are_deletes {
        //     self.has_db_mods(modified, local_abspath)?;
        // }

        let mut modcheck_baton_t = ModcheckBaton::default();
        todo!()
    }

    /// `svn_wc__db_wcroot_parse_local_abspath`
    fn wcroot_parse_local_abspath(&self, local_abspath: &Path) -> Result<(), Error> {
        let original_abspath = local_abspath;

        // we need more logic for finding the database (if it is located
        // outside of the wcroot) and then managing all of that within DB.
        // for now: play quick & dirty.
        if let Some(probe_wcroot) = self.dir_data.get(local_abspath) {
            let wcroot = probe_wcroot;
        }

        Ok(())
    }

    /* Retrieve information about a node.

       For the node implied by LOCAL_ABSPATH from the local filesystem, return
       information in the provided OUT parameters. Each OUT parameter may be
       NULL, indicating that specific item is not requested.

       The information returned comes from the BASE tree, as possibly modified
       by the WORKING and ACTUAL trees.

       If there is no information about the node, then SVN_ERR_WC_PATH_NOT_FOUND
       will be returned.

       The OUT parameters, and their "not available" values are:
         STATUS                  n/a (always available)
         KIND                    svn_node_unknown   (For ACTUAL only nodes)
         REVISION                SVN_INVALID_REVNUM
         REPOS_RELPATH           NULL
         REPOS_ROOT_URL          NULL
         REPOS_UUID              NULL
         CHANGED_REV             SVN_INVALID_REVNUM
         CHANGED_DATE            0
         CHANGED_AUTHOR          NULL
         DEPTH                   svn_depth_unknown
         CHECKSUM                NULL
         TARGET                  NULL

         ORIGINAL_REPOS_RELPATH  NULL
         ORIGINAL_ROOT_URL       NULL
         ORIGINAL_UUID           NULL
         ORIGINAL_REVISION       SVN_INVALID_REVNUM

         LOCK                    NULL

         RECORDED_SIZE           SVN_INVALID_FILESIZE
         RECORDED_TIME       0

         CHANGELIST              NULL
         CONFLICTED              FALSE

         OP_ROOT                 FALSE
         HAD_PROPS               FALSE
         PROPS_MOD               FALSE

         HAVE_BASE               FALSE
         HAVE_MORE_WORK          FALSE
         HAVE_WORK               FALSE

       When STATUS is requested, then it will be one of these values:

         svn_wc__db_status_normal
           A plain BASE node, with no local changes.

         svn_wc__db_status_added
           A node has been added/copied/moved to here. See HAVE_BASE to see
           if this change overwrites a BASE node. Use scan_addition() to resolve
           whether this has been added, copied, or moved, and the details of the
           operation (this function only looks at LOCAL_ABSPATH, but resolving
           the details requires scanning one or more ancestor nodes).

         svn_wc__db_status_deleted
           This node has been deleted or moved away. It may be a delete/move of
           a BASE node, or a child node of a subtree that was copied/moved to
           an ancestor location. Call scan_deletion() to determine the full
           details of the operations upon this node.

         svn_wc__db_status_server_excluded
           The node is versioned/known by the server, but the server has
           decided not to provide further information about the node. This
           is a BASE node (since changes are not allowed to this node).

         svn_wc__db_status_excluded
           The node has been excluded from the working copy tree. This may
           be an exclusion from the BASE tree, or an exclusion in the
           WORKING tree for a child node of a copied/moved parent.

         svn_wc__db_status_not_present
           This is a node from the BASE tree, has been marked as "not-present"
           within this mixed-revision working copy. This node is at a revision
           that is not in the tree, contrary to its inclusion in the parent
           node's revision.

         svn_wc__db_status_incomplete
           The BASE is incomplete due to an interrupted operation.  An
           incomplete WORKING node will be svn_wc__db_status_added.

       If REVISION is requested, it will be set to the revision of the
       unmodified (BASE) node, or to SVN_INVALID_REVNUM if any structural
       changes have been made to that node (that is, if the node has a row in
       the WORKING table).

       If DEPTH is requested, and the node is NOT a directory, then
       the value will be set to svn_depth_unknown.

       If CHECKSUM is requested, and the node is NOT a file, then it will
       be set to NULL.

       If TARGET is requested, and the node is NOT a symlink, then it will
       be set to NULL.

       If TRANSLATED_SIZE is requested, and the node is NOT a file, then
       it will be set to SVN_INVALID_FILESIZE.

       If HAVE_WORK is TRUE, the returned information is from the highest WORKING
       layer. In that case HAVE_MORE_WORK and HAVE_BASE provide information about
       what other layers exist for this node.

       If HAVE_WORK is FALSE and HAVE_BASE is TRUE then the information is from
       the BASE tree.

       If HAVE_WORK and HAVE_BASE are both FALSE and when retrieving CONFLICTED,
       then the node doesn't exist at all.

       If OP_ROOT is requested and the node has a WORKING layer, OP_ROOT will be
       set to true if this node is the op_root for this layer.

       If HAD_PROPS is requested and the node has pristine props, the value will
       be set to TRUE.

       If PROPS_MOD is requested and the node has property modification the value
       will be set to TRUE.

       ### add information about the need to scan upwards to get a complete
       ### picture of the state of this node.

       ### add some documentation about OUT parameter values based on STATUS ??

       ### the TEXT_MOD may become an enumerated value at some point to
       ### indicate different states of knowledge about text modifications.
       ### for example, an "svn edit" command in the future might set a
       ### flag indicating administratively-defined modification. and/or we
       ### might have a status indicating that we saw it was modified while
       ### performing a filesystem traversal.

       All returned data will be allocated in RESULT_POOL. All temporary
       allocations will be made in SCRATCH_POOL.
    */
    /* ### old docco. needs to be incorporated as appropriate. there is
       ### some pending, potential changes to the definition of this API,
       ### so not worrying about it just yet.

       ### if the node has not been committed (after adding):
       ###   revision will be SVN_INVALID_REVNUM
       ###   repos_* will be NULL
       ###   changed_rev will be SVN_INVALID_REVNUM
       ###   changed_date will be 0
       ###   changed_author will be NULL
       ###   status will be svn_wc__db_status_added
       ###   text_mod will be TRUE
       ###   prop_mod will be TRUE if any props have been set
       ###   base_shadowed will be FALSE

       ### if the node is not a copy, or a move destination:
       ###   original_repos_path will be NULL
       ###   original_root_url will be NULL
       ###   original_uuid will be NULL
       ###   original_revision will be SVN_INVALID_REVNUM

       ### note that @a base_shadowed can be derived. if the status specifies
       ### an add/copy/move *and* there is a corresponding node in BASE, then
       ### the BASE has been deleted to open the way for this node.
    */
    /// `svn_wc__db_read_info`
    pub fn read_info<P: AsRef<Path>>(&self, local_abspath: P) -> Result<(), Error> {
        let p = local_abspath.as_ref();
        todo!()
    }
}

/// FIXME: move to enum
// #[derive(Debug, Clone, PartialEq)]
// pub enum LocalModifications {
//     /// 没有本地修改
//     None,
//     /// 只有删除操作
//     OnlyDeletes,
//     /// 包含非删除的修改（添加、修改、替换等）
//     Mixed,
// }
#[derive(Debug, Clone, PartialEq)]
pub struct LocalModsInfo {
    pub has_modifications: bool,
    pub all_edits_are_deletes: bool,
}

/// A baton for use with modcheck_found_entry().
#[derive(Default, Debug)]
struct ModcheckBaton {
    ignore_unversioned: bool,
    /// whether a modification has been found
    found_mod: bool,
    /// Found a not-delete modification
    found_not_delete: bool,
}
