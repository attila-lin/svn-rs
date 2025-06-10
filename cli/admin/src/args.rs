use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AppArgs {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// usage: svnadmin build-repcache REPOS_PATH [-r LOWER[:UPPER]]
    ///
    /// Add missing entries to the representation cache for the repository
    /// at REPOS_PATH. Process data in revisions LOWER through UPPER.
    /// If no revision arguments are given, process all revisions. If only
    /// LOWER revision argument is given, process only that single revision.
    BuildRepcache {},
    /// usage: svnadmin crashtest REPOS_PATH
    ///
    /// Open the repository at REPOS_PATH, then abort, thus simulating
    /// a process that crashes while holding an open repository handle.
    CrashTest {
        #[arg(value_name = "REPOS_PATH")]
        repos_path: String,
    },

    /// usage: svnadmin create REPOS_PATH
    ///
    /// Create a new, empty repository at REPOS_PATH.
    Create {
        #[arg(value_name = "REPOS_PATH")]
        repos_path: String,
    },

    /// Verify the data stored in the repository.
    Verify {
        /// REPOS_PATH
        repos_path: String,
    },
}
