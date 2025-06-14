use clap::Parser;

/// rdump - dump a Subversion repository in a human-readable format
#[derive(Parser, Debug)]
#[command(version, about, long_about, max_term_width = 80)]
pub struct AppArgs {
    /// specify revision number ARG (or X:Y range)
    #[arg(short, long, value_name = "ARG", default_value = "HEAD")]
    pub revision: String,
    /// no progress (only errors) to stderr
    #[arg(short, long)]
    pub quiet: bool,
    /// dump incrementally
    #[arg(long)]
    pub incremental: bool,
    /// skip revision property ARG (e.g., "svn:author")
    #[arg(short, long = "skip-revprop", value_name = "ARG")]
    pub skip_revision_property: Option<String>,

    /// read user configuration files from directory ARG
    #[arg(long, value_name = "ARG")]
    pub config_dir: Option<String>,

    /// specify a username ARG
    #[arg(long, value_name = "ARG")]
    pub username: Option<String>,

    /// specify a password ARG
    #[arg(long, value_name = "ARG")]
    pub password: Option<String>,

    /// read password from stdin
    #[arg(long = "password-from-stdin")]
    pub password_stdin: bool,

    /// do no interactive prompting (default is to prompt
    /// only if standard input is a terminal device
    #[arg(long = "non-interactive")]
    pub non_interactive: bool,

    /// do interactive prompting even if standard input
    /// is not a terminal device
    #[arg(long = "force-interactive")]
    pub force_interactive: bool,

    /// do not cache authentication tokens
    #[arg(long = "no-auth-cache")]
    pub no_auth_cache: bool,

    /// set user configuration option in the format:
    ///     FILE:SECTION:OPTION=[VALUE]
    /// For example:
    ///     servers:global:http-library=serf
    #[arg(long = "config-option", value_name = "FILE:SECTION:OPTION=[VALUE]")]
    pub config_option: Option<String>,

    /// with --non-interactive, accept SSL server
    /// certificates with failures; ARG is comma-separated
    /// list of 'unknown-ca' (Unknown Authority),
    /// 'cn-mismatch' (Hostname mismatch), 'expired'
    /// (Expired certificate), 'not-yet-valid' (Not yet
    /// valid certificate) and 'other' (all other not
    /// separately classified certificate errors).
    #[arg(long = "trust-server-cert-failures", value_name = "ARG")]
    pub trust_server_cert_failures: Option<String>,

    /// read/write file ARG instead of stdin/stdout
    #[arg(short, long, value_name = "ARG")]
    pub file: Option<String>,
}
