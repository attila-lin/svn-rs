use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(version, about, long_about, max_term_width = 80)]
pub struct AppArgs {
    /// Do not display filtering statistics.
    #[arg(short, long)]
    quiet: bool,
    /// Treat the path prefixes as file glob patterns.
    /// Glob special characters are '*' '?' '[]' and '\\'.
    /// Character '/' is not treated specially, so
    /// pattern /*/foo matches paths /a/foo and /a/b/foo.
    #[arg(long)]
    pattern: Option<String>,
    /// Remove revisions emptied by filtering.
    #[arg(long = "drop-empty-revs")]
    drop_empty_revs: bool,
    /// Remove all empty revisions found in dumpstream,
    /// except revision 0.
    #[arg(long = "drop-all-empty-revs")]
    drop_all_empty_revs: bool,
    /// Renumber revisions left after filtering.
    #[arg(long = "renumber-revs")]
    renumber_revs: bool,

    /// Skip missing merge sources.
    #[arg(long = "skip-missing-merge-sources")]
    skip_missing_merge_sources: bool,

    /// Don't filter revision properties.
    #[arg(long = "preserve-revprops")]
    preserve_revprops: bool,

    /// Read additional prefixes, one per line, from file ARG.
    #[arg(long, value_name = "ARG")]
    targets: Option<String>,

    #[command(subcommand)]
    command: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Filter out nodes with given prefixes from dumpstream.
    /// usage: svndumpfilter exclude PATH_PREFIX...
    Exclude {
        #[arg(value_name = "PATH_PREFIX")]
        path_prefix: String,
    },
    /// Filter out nodes without given prefixes from dumpstream.
    /// usage: svndumpfilter include PATH_PREFIX...
    Include {
        #[arg(value_name = "PATH_PREFIX")]
        path_prefix: String,
    },
}
