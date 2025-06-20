use std::collections::HashMap;

use svn_subr::auth::AuthBaton;
use svn_wc::NotifyFunc;
use svn_wc::WcContext;

/// Private client context.
///
/// This is what is actually allocated by svn_client_create_context2(),
/// which then returns the address of the public_ctx member.
/// `svn_client__private_ctx_t`
pub struct PrivateCtx {
    /// Reserved field, always zero, to detect misuse of the private
    ///    context as a public client context.
    magic_null: u64,
    /// Reserved field, always set to a known magic number, to identify
    /// this struct as the private client context.
    magic_id: u64,
    /// Total number of bytes transferred over network across all RA sessions.
    total_progress: usize,
}
/// `svn_client_get_commit_log2_t`
type GetCommitLog = Box<dyn Fn(&Vec<()>) -> Result<(Vec<String>, Vec<String>), ()>>;

/// A client context structure, which holds client specific callbacks,
/// batons, serves as a cache for configuration options, and other various
/// and sundry things.  In order to avoid backwards compatibility problems
/// clients should use svn_client_create_context() to allocate and
/// initialize this structure instead of doing so themselves.
///
/// `svn_client_ctx_t`
pub struct SvnClientCtx {
    private_ctx: PrivateCtx,
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
    /// #svn_config_t *'s. For example, the '~/.subversion/config' file's
    /// contents should have the key "config".  May be left unset (or set to
    /// NULL) to use the built-in default settings and not use any configuration.
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

impl SvnClientCtx {
    const CLIENT_CTX_MAGIC: u64 = 0xDEADBEEF600DF00D;

    /// Create a new client context.
    /// `svn_client_create_context2`
    pub fn new(config: &HashMap<String, String>) -> Self {
        let mut ret = Self {
            private_ctx: PrivateCtx {
                magic_null: 0,
                magic_id: Self::CLIENT_CTX_MAGIC,
                total_progress: 0,
            },

            config,
            wc_ctx: WcContext::new(),
        };
        ret
    }
}
