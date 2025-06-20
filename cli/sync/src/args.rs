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
