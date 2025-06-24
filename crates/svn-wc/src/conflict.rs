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
