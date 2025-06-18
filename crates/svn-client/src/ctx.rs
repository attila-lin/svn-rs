use std::collections::HashMap;

use svn_wc::NotifyFunc;
use svn_wc::WcContext;

/// A client context structure, which holds client specific callbacks,
/// batons, serves as a cache for configuration options, and other various
/// and sundry things.  In order to avoid backwards compatibility problems
/// clients should use svn_client_create_context() to allocate and
/// initialize this structure instead of doing so themselves.
///
/// `svn_client_ctx_t`
pub struct SvnClientCtx {
    /// main authentication baton.
    auth_baton: AuthBaton,

    /// notification callback function.
    /// This will be called by notify_func2() by default.
    /// @deprecated Provided for backward compatibility with the 1.1 API.
    /// Use @c notify_func2 instead.
    notify_func: NotifyFunc,
    /// Log message callback function.  NULL means that Subversion
    /// should try not attempt to fetch a log message.
    /// @deprecated Provided for backward compatibility with the 1.2 API.
    /// Use @c log_msg_func2 instead.
    log_msg_fun: GetCommitLog,

    /// a hash mapping of <tt>const char *</tt> configuration file names to
    ///  * #svn_config_t *'s. For example, the '~/.subversion/config' file's
    ///  * contents should have the key "config".  May be left unset (or set to
    ///  * NULL) to use the built-in default settings and not use any configuration.
    config: HashMap<String, String>,
    /// a callback to be used to see if the client wishes to cancel the running
    /// operation.
    cancel_trait: Box<dyn CancelTrait>,

    /// MIME types map.
    mimetypes_map: HashMap<String, String>,

    /// Conflict resolution callback and baton, if available.
    conflict_trait: Option<Box<dyn ConflictTrait>>,

    /// Custom client name string, or NULL.
    client_name: Option<String>,
    /// A working copy context for the client operation to use.
    /// This is initialized by svn_client_create_context() and should never
    /// be @c NULL.
    wc_ctx: WcContext,
}
