use clap::Parser;
use clap::Subcommand;

/// general usage: svnsync SUBCOMMAND DEST_URL  [ARGS & OPTIONS ...]
///
/// Subversion repository replication tool.
#[derive(Debug, Parser)]
#[clap(version, about, long_about)]
pub struct AppArgs {
    /// print as little as possible
    #[arg(short, long)]
    quiet: bool,
    /// operate on revision ARG (or range ARG1:ARG2)
    /// A revision argument can be one of:
    /// - NUMBER: revision number
    /// - 'HEAD': the latest revision
    #[arg(short, long, default_value = "HEAD")]
    pub revision: String,

    /// allow a non-empty destination repository
    #[arg(long = "allow-non-empty")]
    allow_non_empty: bool,

    /// don't copy unchanged revision properties
    #[arg(long = "skip-unchanged")]
    skip_unchanged: bool,

    /// do no interactive prompting (default is to prompt
    /// only if standard input is a terminal device)
    #[arg(long = "non-interactive")]
    non_interactive: bool,

    /// with --non-interactive, accept SSL
    /// server certificates with failures.
    /// ARG is a comma-separated list of:
    /// - 'unknown-ca': Unknown Authority
    /// - 'cn-mismatch': Hostname mismatch
    /// - 'expired': Expired certificate
    /// - 'not-yet-valid': Not yet valid certificate
    /// - 'other': all other not separately classified certificate errors
    ///
    /// Applied to the source URL.
    #[arg(long = "source-trust-server-cert-failures", value_name = "ARG")]
    source_trust_server_cert_failures: Option<String>,

    /// Like
    /// --source-trust-server-cert-failures,
    /// but applied to the destination URL.
    #[arg(long = "sync-trust-server-cert-failures", value_name = "ARG")]
    sync_trust_server_cert_failures: Option<String>,

    /// connect to source repository with username ARG
    #[arg(long = "source-username", value_name = "ARG")]
    source_username: Option<String>,

    /// connect to sync repository with username ARG
    #[arg(long = "sync-username", value_name = "ARG")]
    sync_username: Option<String>,

    /// connect to source repository with password ARG
    #[arg(long = "source-password", value_name = "ARG")]
    source_password: Option<String>,

    /// connect to sync repository with password ARG
    #[arg(long = "sync-password", value_name = "ARG")]
    sync_password: Option<String>,

    /// read user configuration files from directory ARG
    #[arg(long = "config-dir", value_name = "ARG")]
    config_dir: Option<String>,

    /// set user configuration option in the format:
    ///
    ///   FILE:SECTION:OPTION=[VALUE]
    ///
    /// For example:
    ///
    ///   servers:global:http-library=serf
    #[arg(long = "config-option", value_name = "FILE:SECTION:OPTION=[VALUE]")]
    config_option: Option<String>,

    #[command(subcommand)]
    pub command: Option<SubCommand>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// usage: svnsync initialize DEST_URL SOURCE_URL
    ///
    /// Initialize a destination repository for synchronization from
    /// another repository.
    ///
    /// If the source URL is not the root of a repository, only the
    /// specified part of the repository will be synchronized.
    /// The destination URL must point to the root of a repository which
    /// has been configured to allow revision property changes.  In
    /// the general case, the destination repository must contain no
    /// committed revisions.  Use --allow-non-empty to override this
    /// restriction, which will cause svnsync to assume that any revisions
    /// already present in the destination repository perfectly mirror
    /// their counterparts in the source repository.  (This is useful
    /// when initializing a copy of a repository as a mirror of that same
    /// repository, for example.)
    ///
    /// You should not commit to, or make revision property changes in,
    /// the destination repository by any method other than 'svnsync'.
    /// In other words, the destination repository should be a read-only
    /// mirror of the source repository.
    #[command(name = "initialize")]
    Initialize {},

    /// usage: svnsync info DEST_URL
    ///
    /// Print information about the synchronization destination repository
    /// located at DEST_URL.
    Info {
        #[arg(value_name = "DEST_URL")]
        dest_url: String,
    },
}
