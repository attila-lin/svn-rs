pub mod patch;

pub mod parse_diff;

pub mod tree;

/// The separator string used below the "Index:" or similar line of
/// Subversion's Unidiff-like diff format.
const EQUAL_STR: &str = "===================================================================";

/// The separator string used below the "Properties on ..." line of
/// Subversion's Unidiff-like diff format.
const UNDER_STR: &str = "___________________________________________________________________";

/// There are four types of datasources.  In GNU diff3 terminology,
/// the first three types correspond to the phrases "older", "mine",
/// and "yours".
///
/// `svn_diff_datasource_e`
pub enum SvnDiffDatasource {
    /// The oldest form of the data.
    Original,
    /// The same data, but potentially changed by the user.
    Modified,
    /// The latest version of the data, possibly different than the
    /// user's modified version.
    Latest,
    /// The common ancestor of original and modified.
    Ancestor,
}

/// A vtable for reading data from the three datasources.
///
/// `svn_diff_fns2_t`
pub trait SvnDiffFnsTrait {
    /// Open the datasources of type @a datasources.
    fn datasources_open();

    /// Close the datasource of type @a datasource.
    fn datasource_close();

    /// Get the next "token" from the datasource of type @a datasource.
    /// Return a "token" in @a *token.   Return a hash of "token" in @a *hash.
    /// Leave @a token and @a hash untouched when the datasource is exhausted.
    fn datasource_get_next_token();

    /// A function for ordering the tokens, resembling 'strcmp' in functionality.
    /// @a compare should contain the return value of the comparison:
    /// If @a ltoken and @a rtoken are "equal", return 0.  If @a ltoken is
    /// "less than" @a rtoken, return a number < 0.  If @a ltoken  is
    /// "greater than" @a rtoken, return a number > 0.
    fn token_compare();

    /// Free @a token from memory, the diff algorithm is done with it.
    fn token_discard();

    /// Free *all* tokens from memory, they're no longer needed.
    fn token_discard_all();
}
