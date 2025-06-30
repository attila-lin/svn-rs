/// A callback used in merge, update and switch for resolving conflicts
/// during the application of a tree delta to a working copy.
///
/// @a description describes the exact nature of the conflict, and
/// provides information to help resolve it.  @a baton is a closure
/// object; it should be provided by the implementation, and passed by
/// the caller.  When finished, the callback signals its resolution by
/// returning a structure in @a *result, which should be allocated in
/// @a result_pool.  (See #svn_wc_conflict_result_t.)  @a scratch_pool
/// should be used for any temporary allocations.
///
/// The values #svn_wc_conflict_choose_mine_conflict and
/// #svn_wc_conflict_choose_theirs_conflict are not legal for conflicts
/// in binary files or binary properties.
///
/// Implementations of this callback are free to present the conflict
/// using any user interface.  This may include simple contextual
/// conflicts in a file's text or properties, or more complex
/// 'tree'-based conflicts related to obstructed additions, deletions,
/// and edits.  The callback implementation is free to decide which
/// sorts of conflicts to handle; it's also free to decide which types
/// of conflicts are automatically resolvable and which require user
/// interaction.
///
/// @since New in 1.7.
///
/// `svn_wc_conflict_resolver_func2_t`
pub type WcConflictResolverFunc = Box<dyn Fn(&str, &str, &str) -> Result<(), String> + Send + Sync>;

/// The way in which the conflict callback chooses a course of action.
///  *
///  * @since New in 1.5.
/// `svn_wc_conflict_choice_t`
pub enum ConflictChoice {
    /// Undefined; for private use only.
    ///    This value must never be returned in svn_wc_conflict_result_t,
    ///    but a separate value, unequal to all other pre-defined values may
    ///    be useful in conflict resolver implementations to signal that no
    ///    choice is made yet.
    /// * @since New in 1.9
    Undefined = -1,
    /// Don't resolve the conflict now.  Let libsvn_wc mark the path
    /// 'conflicted', so user can run 'svn resolved' later.
    Postpone = 0,

    /// If there were files to choose from, select one as a way of
    ///    resolving the conflict here and now.  libsvn_wc will then do the
    ///    work of "installing" the chosen file.
    Base = 1,
    TheirsFull = 2,
    MineFull = 3,
    TheirsConflict = 4,
    MineConflict = 5,
    Merged = 6,
    Unspecified = 7,
}

/** The final result returned by #svn_wc_conflict_resolver_func_t.
 *
 * @note Fields may be added to the end of this structure in future
 * versions.  Therefore, to preserve binary compatibility, users
 * should not directly allocate structures of this type.  Instead,
 * construct this structure using svn_wc_create_conflict_result()
 * below.
 *
 * @since New in 1.5.
 */
/// `svn_wc_conflict_result_t`
pub struct ConflictResult {
    /** A choice to either delay the conflict resolution or select a
    particular file to resolve the conflict. */
    choice: ConflictChoice,
    /** If not NULL, this is a path to a file which contains the client's
    (or more likely, the user's) merging of the three values in
    conflict.  libsvn_wc accepts this file if (and only if) @c choice
    is set to #svn_wc_conflict_choose_merged.*/
    merged_file: Option<String>,
    /** If true, save a backup copy of merged_file (or the original
    merged_file from the conflict description, if merged_file is
    NULL) in the user's working copy. */
    save_merged: bool,

    /** If not NULL, this is the new merged property, used when choosing
     * #svn_wc_conflict_choose_merged. This value is preferred over using
     * merged_file.
     *
     * @since New in 1.9.
     */
    merged_value: Option<String>,
}
impl ConflictResult {
    // Constructor for the result-structure returned by conflict callbacks.
    /// `svn_wc_create_conflict_result`
    pub fn create(choice: ConflictChoice, merged_file: Option<String>) -> Self {
        Self {
            choice,
            merged_file,
            save_merged: false,
            merged_value: None,
        }
    }
}
