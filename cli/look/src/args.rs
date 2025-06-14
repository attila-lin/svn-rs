use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about, max_term_width = 80)]
pub struct AppArgs {
    /// show details for copies
    #[arg(long = "copy-info")]
    copy_info: bool,

    /// print differences against the copy source
    #[arg(long = "diff-copy-from")]
    diff_copy_from: bool,

    /// show full paths instead of indenting them
    #[arg(long = "full-paths")]
    full_paths: bool,

    /// maximum number of history entries
    #[arg(short, long, value_name = "N")]
    limit: Option<usize>,

    /// do not print differences for added files
    #[arg(long = "no-diff-added")]
    no_diff_added: bool,

    /// do not print differences for deleted files
    #[arg(long = "no-diff-deleted")]
    no_diff_deleted: bool,

    /// use ARG as diff command
    #[arg(long = "diff-cmd", value_name = "ARG")]
    diff_cmd: Option<String>,

    /// ignore properties during the operation
    #[arg(long = "ignore-properties")]
    ignore_properties: bool,

    /// show only properties during the operation
    #[arg(long = "properties-only")]
    properties_only: bool,

    /// size of the extra in-memory cache in MB used to
    /// minimize redundant operations. Default: 16.
    /// [used for FSFS repositories only]
    #[arg(short = 'M', long, value_name = "N", default_value_t = 16)]
    cache_size: u16,

    /// do not output the trailing newline
    #[arg(short, long)]
    no_newline: bool,
    /// operate on single directory only
    #[arg(short = 'N', long = "non-recursive")]
    non_recursive: bool,

    /// specify revision number ARG
    #[arg(short, long, value_name = "ARG")]
    revision: Option<String>,

    /// operate on a revision property (use with -r or -t)
    #[arg(long = "revprop", value_name = "ARG")]
    revision_property: Option<String>,

    /// show node revision ids for each path
    #[arg(long = "show-ids")]
    show_ids: bool,

    /// show path's inherited properties
    #[arg(long = "show-inherited-props")]
    show_inherited_props: bool,

    /// specify transaction name ARG
    #[arg(short, long, value_name = "ARG")]
    transaction: Option<String>,

    /// output in XML format
    #[arg(long)]
    xml: bool,

    /// Specify differencing options for external diff or
    /// internal diff. Default: '-u'. Options are
    /// separated by spaces. Internal diff takes:
    ///  -u, --unified: Show 3 lines of unified context
    ///  -b, --ignore-space-change: Ignore changes in
    ///    amount of white space
    ///  -w, --ignore-all-space: Ignore all white space
    ///  --ignore-eol-style: Ignore changes in EOL style
    ///  -U ARG, --context ARG: Show ARG lines of context
    ///  -p, --show-c-function: Show C function name
    #[arg(short = 'x', long = "extensions")]
    extensions: Vec<String>,

    /// no progress (only errors) to stderr
    #[arg(short, long)]
    quiet: bool,

    #[command(subcommand)]
    command: Option<SubCommand>,
}

#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    /// usage: svnlook author REPOS_PATH
    /// Print the author.
    Author {
        #[arg(value_name = "REPOS_PATH")]
        repos_path: String,
    },

    /// usage: svnlook cat REPOS_PATH FILE_PATH
    /// Print the contents of a file.  Leading '/' on FILE_PATH is optional.
    Cat {
        #[arg(value_name = "REPOS_PATH")]
        repos_path: String,
        #[arg(value_name = "FILE_PATH")]
        file_path: String,
    },

    /// usage: svnlook changed REPOS_PATH
    /// Print the paths that were changed.
    Changed {
        #[arg(value_name = "REPOS_PATH")]
        repos_path: String,
    },

    /// usage: svnlook date REPOS_PATH
    /// Print the datestamp.
    Date {
        #[arg(value_name = "REPOS_PATH")]
        repos_path: String,
    },

    /// usage: svnlook diff REPOS_PATH
    /// Print GNU-style diffs of changed files and properties.
    Diff {
        #[arg(value_name = "REPOS_PATH")]
        repos_path: String,
    },

    /// usage: 1. svnlook proplist REPOS_PATH PATH_IN_REPOS
    ///        2. svnlook proplist --revprop REPOS_PATH
    /// List the properties of a path in the repository, or
    /// "with the --revprop option, revision properties."
    /// "With -v, show the property values too."
    Proplist {
        #[arg(value_name = "REPOS_PATH")]
        repos_path: String,
        #[arg(value_name = "PATH_IN_REPOS")]
        path_in_repos: String,
    },

    /// usage: svnlook tree REPOS_PATH [PATH_IN_REPOS]
    /// "Print the tree, starting at PATH_IN_REPOS (if supplied, at the root
    /// of the tree otherwise), optionally showing node revision ids.
    Tree {
        #[arg(value_name = "REPOS_PATH")]
        repos_path: String,
        #[arg(value_name = "PATH_IN_REPOS")]
        path_in_repos: Option<String>,
    },

    /// usage: svnlook uuid REPOS_PATH
    /// Print the UUID of the repository.
    Uuid {
        #[arg(value_name = "REPOS_PATH")]
        repos_path: String,
    },

    /// usage: svnlook youngest REPOS_PATH
    /// Print the youngest revision number.
    Youngest {
        #[arg(value_name = "REPOS_PATH")]
        repos_path: String,
    },
}
