use std::path::Path;

use svn_subr::SvnConfig;
use svn_types::RevisionNumber;

use crate::Error;
use crate::WcDb;
use crate::status::RevisionStatus;

///  Context handling
///
/// `svn_wc_context_t`
#[derive(Debug)]
pub struct WcContext {
    /// The wc_db handle for this working copy.
    db: WcDb,

    /// Close the DB when we destroy this context?
    ///
    /// This is used inside backward compat wrappers, and should only be
    /// modified by the proper create() functions.
    close_db_on_destroy: bool,
}

impl WcContext {
    /// Create a new working copy context.
    pub fn new() -> Self {
        WcContext {
            db: WcDb::open().unwrap(),
            close_db_on_destroy: true,
        }
    }

    /// Create a context for the working copy, and return it in @a *wc_ctx.
    /// This context is not associated with a particular working copy, but as operations
    /// are performed, will load the appropriate working copy information.
    ///
    /// @a config should hold the various configuration options that may apply to
    /// this context.  It should live at least as long as @a result_pool.  It may
    /// be @c NULL.
    ///
    /// The context will be allocated in @a result_pool, and will use @a
    /// result_pool for any internal allocations requiring the same longevity as
    /// the context.  The context will be automatically destroyed, and its
    /// resources released, when @a result_pool is cleared, or it may be manually
    /// destroyed by invoking svn_wc_context_destroy().
    ///
    /// Use @a scratch_pool for temporary allocations.  It may be cleared
    /// immediately upon returning from this function.
    ///
    /// `svn_wc_context_create`
    pub fn create(config: Option<SvnConfig>) -> Self {
        todo!()
    }

    /// create a new working copy context with db
    ///
    /// Just like svn_wc_context_create(), only use the provided DB to construct
    /// the context.
    ///
    /// Even though DB is not allocated from the same pool at *WC_CTX, it is
    /// expected to remain open throughout the life of *WC_CTX.
    ///
    /// `svn_wc__context_create_with_db`
    pub fn new_with_db(db: WcDb, config: SvnConfig) -> Self {
        WcContext {
            db,
            close_db_on_destroy: false,
        }
    }

    /// Set @a *result_p to point to a new #svn_wc_revision_status_t structure
    /// containing a summary of the revision range and status of the working copy
    /// at @a local_abspath (not including "externals").  @a local_abspath must
    /// be absolute. Return SVN_ERR_WC_PATH_NOT_FOUND if @a local_abspath is not
    /// a working copy path.
    ///
    /// Set @a (*result_p)->min_rev and @a (*result_p)->max_rev respectively to the
    /// lowest and highest revision numbers in the working copy.  If @a committed
    /// is TRUE, summarize the last-changed revisions, else the base revisions.
    ///
    /// Set @a (*result_p)->switched to indicate whether any item in the WC is
    /// switched relative to its parent.  If @a trail_url is non-NULL, use it to
    /// determine if @a local_abspath itself is switched.  It should be any trailing
    /// portion of @a local_abspath's expected URL, long enough to include any parts
    /// that the caller considers might be changed by a switch.  If it does not
    /// match the end of @a local_abspath's actual URL, then report a "switched"
    /// status.
    ///
    /// Set @a (*result_p)->modified to indicate whether any item is locally
    /// modified.
    ///
    /// If @a cancel_func is non-NULL, call it with @a cancel_baton to determine
    /// if the client has canceled the operation.
    ///
    /// Allocate *result_p in @a result_pool, use @a scratch_pool for temporary
    /// allocations.
    ///
    /// @a wc_ctx should be a valid working copy context.
    ///
    /// `svn_wc_revision_status2`
    pub fn revision_status(
        &self,
        local_abspath: &str,
        trail_url: &Option<String>,
        commited: bool,
    ) -> Result<RevisionStatus, Error> {
        let local_abspath = Path::new(local_abspath);
        if !local_abspath.is_absolute() {
            return Err(Error::NotAbsolutePath(
                local_abspath.to_string_lossy().to_string(),
            ));
        }

        let status = self
            .db
            .revision_status(local_abspath, trail_url, commited)?;

        if !status.modified {
            let all_edits_are_deletes = self.db.node_has_local_mods(local_abspath, true)?;
        }

        Ok(status)
    }

    /// Get the changed revision, date and author for @a local_abspath using @a
    /// wc_ctx.  Allocate the return values in @a result_pool; use @a scratch_pool
    /// for temporary allocations.  Any of the return pointers may be @c NULL, in
    /// which case they are not set.
    ///
    /// If @a local_abspath is not in the working copy, return
    /// @c SVN_ERR_WC_PATH_NOT_FOUND.
    ///
    /// `svn_wc__node_get_changed_info`
    pub fn get_changed_info(
        &self,
        local_abspath: &str,
    ) -> Result<(RevisionNumber, i64, &str), Error> {
        self.db.read_info(local_abspath).map_err(Error::from)?;
        todo!()
    }
}
