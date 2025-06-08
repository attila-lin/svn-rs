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
    /// Add missing entries to the representation cache for the repository
    /// at REPOS_PATH. Process data in revisions LOWER through UPPER.
    /// If no revision arguments are given, process all revisions. If only
    /// LOWER revision argument is given, process only that single revision.
    BuildRepcache {},

    /// Create a new, empty repository at REPOS_PATH.
    Create {
        /// REPOS_PATH
        repos_path: String,
    },

    /// Verify the data stored in the repository.
    Verify {
        /// REPOS_PATH
        repos_path: String,
    },
}
