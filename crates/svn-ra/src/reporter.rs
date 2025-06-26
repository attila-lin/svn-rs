use svn_types::Depth;
use svn_types::SvnError;

/// * The update Reporter.
///  *
///  * A vtable structure which allows a working copy to describe a subset
///  * (or possibly all) of its working-copy to an RA layer, for the
///  * purposes of an update, switch, status, or diff operation.
///  *
///  * Paths for report calls are relative to the target (not the anchor)
///  * of the operation.  Report calls must be made in depth-first order:
///  * parents before children, all children of a parent before any
///  * siblings of the parent.  The first report call must be a set_path
///  * with a @a path argument of "" and a valid revision.  (If the target
///  * of the operation is locally deleted or missing, use the anchor's
///  * revision.)  If the target of the operation is deleted or switched
///  * relative to the anchor, follow up the initial set_path call with a
///  * link_path or delete_path call with a @a path argument of "" to
///  * indicate that.  In no other case may there be two report
///  * descriptions for the same path.  If the target of the operation is
///  * a locally added file or directory (which previously did not exist),
///  * it may be reported as having revision 0 or as having the parent
///  * directory's revision.
///  *
///  * @since New in 1.5.`
///
/// `svn_ra_reporter3_t`
pub trait Reporter {
    /// Describe a working copy path as being at a particular revision and having specific depth.
    ///
    /// # Arguments
    /// * `path` - Path relative to the RA session URL
    /// * `revision` - The revision number (can be None for locally added paths or excluded paths)
    /// * `depth` - The depth of the path (files, immediates, infinity, etc.)
    /// * `start_empty` - If true and path is a directory, assume it has no entries or props
    /// * `lock_token` - Optional lock token for the path in the working copy
    ///
    /// # Returns
    /// Result indicating success or error
    fn set_path(
        &mut self,
        path: &str,
        revision: Option<i64>,
        depth: Depth,
        start_empty: bool,
        lock_token: Option<&str>,
    ) -> Result<(), SvnError>;

    /// Describe a working copy path as missing (deleted).
    ///
    /// # Arguments
    /// * `path` - Path relative to the RA session URL that is missing
    ///
    /// # Returns
    /// Result indicating success or error
    fn delete_path(&mut self, path: &str) -> Result<(), SvnError>;

    /// Link a working copy path to a different repository URL and revision.
    /// This is used for switched paths that point to different locations in the repository.
    ///
    /// # Arguments
    /// * `path` - Working copy path relative to the report root
    /// * `url` - Repository URL that this path actually reflects
    /// * `revision` - The revision number at the linked URL
    /// * `depth` - The depth of the linked path
    /// * `start_empty` - If true and path is a directory, assume it has no entries or props
    /// * `lock_token` - Optional lock token for the path in the working copy
    ///
    /// # Returns
    /// Result indicating success or error
    fn link_path(
        &mut self,
        path: &str,
        url: &str,
        revision: Option<i64>,
        depth: Depth,
        start_empty: bool,
        lock_token: Option<&str>,
    ) -> Result<(), SvnError>;

    /// Finish the state report. Any directories or files not explicitly set
    /// are assumed to be at the baseline revision.
    /// No other reporting functions should be called after this.
    ///
    /// # Returns
    /// Result indicating success or error
    fn finish_report(&mut self) -> Result<(), SvnError>;

    /// Abort the report due to an error. This should cause any filesystem
    /// transaction to be aborted and cleaned up.
    /// No other reporting functions should be called after this.
    ///
    /// # Returns
    /// Result indicating success or error
    fn abort_report(&mut self) -> Result<(), SvnError>;
}
