use std::collections::HashMap;

/// The main auth baton.
///
/// `svn_auth_baton_t`
#[derive(Debug, Default)]
pub struct AuthBaton {
    /* run-time parameters needed by providers. */
    parameters: HashMap<String, String>,
    slave_parameters: HashMap<String, String>,

    /// runtime credentials cache.
    creds_cache: HashMap<String, String>,
}

/// Abstracted iteration baton
pub struct AuthIterstate {
    /// the table being searched
    table: Vec<AuthProviderObject>,
    /// the current provider (row)
    provider_index: usize,
    /// did we get the provider's first creds?
    got_first: bool,
    /// the original auth_baton
    auth_baton: AuthBaton,
    parameters: HashMap<String, String>,
}

/// A provider object, ready to be put into an array and given to
/// svn_auth_open().
///
/// `svn_auth_provider_object_t`
struct AuthProviderObject {
    provider_baton: (),
}
