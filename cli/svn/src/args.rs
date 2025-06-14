use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(version, about, long_about, max_term_width = 80)]
pub struct AppArgs {
    /// force operation to run
    #[arg(long)]
    force: bool,
    /// force validity of log message source
    #[arg(long = "force-log")]
    force_log: bool,

    /// specify log message ARG
    #[arg(short, long, value_name = "ARG")]
    message: Option<String>,

    /// print nothing, or only summary information
    #[arg(short, long)]
    quiet: bool,
    /// descend recursively, same as --depth=infinity
    #[arg(short = 'R', long)]
    recursive: bool,
    /// obsolete
    #[arg(short = 'N', long = "non-recursive")]
    non_recursive: bool,
    /// show human-readable output
    #[arg(short = 'H', long = "human-readable")]
    human_readable: bool,

    /// the change made by revision ARG (like -r ARG-1:ARG)
    ///
    ///     If ARG is negative this is like -r ARG:ARG-1
    ///
    /// If ARG is of the form ARG1-ARG2 then this is like
    ///
    ///     ARG1:ARG2, where ARG1 is inclusive
    #[arg(short, long, value_name = "ARG", default_value = "HEAD")]
    change: String,

    /// ARG (some commands also take ARG1:ARG2 range)
    ///     A revision argument can be one of:
    ///         NUMBER       revision number
    ///         '{' DATE '}' revision at start of the date
    ///         'HEAD'       latest in repository
    ///         'BASE'       base rev of item's working copy
    ///         'COMMITTED'  last commit at or before BASE
    ///         'PREV'       revision just before COMMITTED
    #[arg(short, long, value_name = "ARG", default_value = "BASE")]
    revision: String,

    /// read log message from file ARG
    #[arg(short = 'F', long, value_name = "ARG")]
    file: Option<String>,

    /// give output suitable for concatenation
    #[arg(long)]
    incremental: bool,

    /// treat value as being in charset encoding ARG
    #[arg(long)]
    encoding: Option<EncodeType>,

    /// print extra information
    #[arg(short, long)]
    verbose: bool,

    /// display update information
    #[arg(long = "show-updates")]
    show_updates: bool,

    /// specify a username ARG
    #[arg(long, value_name = "ARG")]
    username: Option<String>,
    /// specify a password ARG (caution: on many operating
    /// systems, other users will be able to see this)
    #[arg(long, value_name = "ARG")]
    password: Option<String>,
    /// read password from stdin
    #[arg(long = "password-from-stdin")]
    password_stdin: bool,

    /// Specify differencing options for external diff or
    /// internal diff or blame. Default: '-u'.
    ///
    /// Options are separated by spaces. Internal diff and blame take:
    ///
    ///   -u, --unified: Show 3 lines of unified context
    ///   -b, --ignore-space-change: Ignore changes in amount of white space
    ///   -w, --ignore-all-space: Ignore all white space
    ///   --ignore-eol-style: Ignore changes in EOL style
    ///   -U ARG, --context ARG: Show ARG lines of context
    ///   -p, --show-c-function: Show C function name
    #[arg(short = 'x', long, value_name = "ARG", default_value = "-u")]
    extensions: String,

    /// pass contents of file ARG as additional args
    #[arg(long, value_name = "ARG")]
    targets: Option<String>,

    /// limit operation by depth ARG ('empty', 'files',
    /// 'immediates', or 'infinity')
    #[arg(long, value_name = "ARG")]
    depth: Option<DepthType>,

    /// set new working copy depth to ARG ('exclude',
    /// 'empty', 'files', 'immediates', or 'infinity')
    #[arg(long = "set-depth", value_name = "ARG")]
    set_depth: Option<DepthType>,

    /// output in XML
    #[arg(long)]
    xml: bool,
    // FIXME: output in JSON
    /// do not cross copies while traversing history
    #[arg(long = "stop-on-copy")]
    stop_on_copy: bool,

    /// disregard default and svn:ignore and
    /// svn:global-ignores property ignores
    #[arg(long = "no-ignore")]
    no_ignore: bool,

    /// do not cache authentication tokens
    #[arg(long = "no-auth-cache")]
    no_auth_cache: bool,

    /// with --non-interactive, accept SSL server
    /// certificates with failures; ARG is comma-separated
    /// list of 'unknown-ca' (Unknown Authority),
    /// 'cn-mismatch' (Hostname mismatch), 'expired'
    /// (Expired certificate), 'not-yet-valid' (Not yet
    /// valid certificate) and 'other' (all other not
    /// separately classified certificate errors).
    #[arg(long = "trust-server-cert-failures", value_name = "ARG")]
    trust_server_cert_failures: Option<Vec<ServerCertFailure>>,

    /// do no interactive prompting (default is to prompt
    /// only if standard input is a terminal device)
    #[arg(long = "non-interactive")]
    non_interactive: bool,

    /// do interactive prompting even if standard input
    /// is not a terminal device
    #[arg(long = "force-interactive")]
    force_interactive: bool,

    /// try operation but make no changes
    #[arg(long = "dry-run")]
    dry_run: bool,

    /// disable merge tracking; diff nodes as if related
    #[arg(long = "ignore-ancestry")]
    ignore_ancestry: bool,

    /// ignore externals definitions
    #[arg(long = "ignore-externals")]
    ignore_externals: bool,

    /// use ARG as merge command
    #[arg(long = "diff3-cmd", value_name = "ARG")]
    diff3_cmd: Option<String>,

    /// use ARG as external editor
    #[arg(long = "editor-cmd", value_name = "ARG")]
    editor_cmd: Option<String>,

    /// merge only mergeinfo differences
    #[arg(long = "record-only")]
    record_only: bool,

    /// use ARG as the older target
    #[arg(long, value_name = "ARG")]
    old: Option<String>,
    /// use ARG as the newer target
    #[arg(long, value_name = "ARG")]
    new: Option<String>,

    /// operate on a revision property (use with -r)
    #[arg(long = "revprop")]
    revision_property: bool,

    /// relocate via URL-rewriting
    #[arg(long)]
    relocate: bool,
    /// read user configuration files from directory ARG
    #[arg(long = "config-dir", value_name = "ARG")]
    config_dir: Option<String>,

    /// set user configuration option in the format:
    ///     FILE:SECTION:OPTION=[VALUE]
    /// For example:
    ///     servers:global:http-library=serf
    #[arg(long = "config-option")]
    config_option: Option<String>,

    /// enable automatic properties
    #[arg(long = "auto-props")]
    auto_props: bool,

    /// disable automatic properties
    #[arg(long = "no-auto-props")]
    no_auto_props: bool,

    /// use a different EOL marker than the standard
    /// system marker for files with the svn:eol-style
    /// property set to 'native'.
    /// ARG may be one of 'LF', 'CR', 'CRLF'
    #[arg(long = "native-eol", value_name = "ARG")]
    native_eol: Option<EolType>,

    /// maximum number of log entries
    #[arg(short, long, value_name = "ARG")]
    limit: Option<usize>,

    /// don't unlock the targets
    #[arg(long = "no-unlock")]
    no_unlock: bool,
    /// remove changelist association
    #[arg(long)]
    remove: bool,

    /// operate only on members of changelist ARG
    #[arg(long, value_name = "ARG")]
    changelist: Option<String>,

    /// don't delete changelists after commit
    #[arg(long = "keep-changelists")]
    keep_changelists: bool,

    /// keep path in working copy
    #[arg(long = "keep-local")]
    keep_local: bool,

    /// retrieve all revision properties
    #[arg(long = "with-all-revprops")]
    with_all_revision_properties: bool,

    /// retrieve no revision properties
    #[arg(long = "with-no-revprops")]
    with_no_revision_properties: bool,

    /// set revision property ARG in new revision
    ///
    /// using the name[=value] format
    #[arg(long = "with-revprop", value_name = "NAME[=VALUE]")]
    with_revision_property: Option<String>,

    /// make intermediate directories
    #[arg(long)]
    parents: bool,

    /// use/display additional information from merge
    /// history
    #[arg(short = 'g', long = "use-merge-history")]
    use_merge_history: bool,

    /// specify automatic conflict resolution action
    /// ('postpone', 'working', 'base', 'mine-conflict',
    /// 'theirs-conflict', 'mine-full', 'theirs-full',
    /// 'edit', 'launch', 'recommended') (shorthand:
    /// 'p', 'mc', 'tc', 'mf', 'tf', 'e', 'l', 'r')
    #[arg(long)]
    accept: Option<String>,

    /// specify which collection of revisions to display
    /// ('merged', 'eligible')
    #[arg(long = "show-revs")]
    show_revisions: Option<CollectionType>,

    /// number of leading path components to strip from
    /// paths parsed from the patch file. --strip 0
    /// is the default and leaves paths unmodified.
    /// --strip 1 would change the path
    /// 'doc/fudge/crunchy.html' to 'fudge/crunchy.html'.
    /// --strip 2 would leave just 'crunchy.html'
    /// The expected component separator is '/' on all
    /// platforms. A leading '/' counts as one component.
    #[arg(long, value_name = "ARG", default_value_t = 0)]
    strip: usize,

    /// don't expand keywords
    #[arg(long = "ignore-keywords")]
    ignore_keywords: bool,

    /// apply the unidiff in reverse
    #[arg(long = "reverse-diff")]
    reverse_diff: bool,

    /// ignore whitespace during pattern matching
    #[arg(long = "ignore-whitespace")]
    ignore_whitespace: bool,

    /// produce diff output
    #[arg(long)]
    diff: bool,

    /// use ARG as diff command
    #[arg(long = "diff-cmd", value_name = "ARG")]
    diff_cmd: Option<String>,

    /// override diff-cmd specified in config file
    #[arg(long = "internal-diff")]
    internal_diff: bool,

    /// do not print differences for added files
    #[arg(long = "no-diff-added")]
    no_diff_added: bool,

    /// do not print differences for deleted files
    #[arg(long = "no-diff-deleted")]
    no_diff_deleted: bool,

    /// don't diff copied or moved files with their source
    #[arg(long = "show-copies-as-adds")]
    show_copies_as_adds: bool,

    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum EncodeType {
    Message,
    Filedata,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum DepthType {
    Empty,
    Files,
    Immediates,
    Infinity,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum EolType {
    Lf,
    Cr,
    CrLf,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum CollectionType {
    Merged,
    Eligible,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum ServerCertFailure {
    /// Unknown Authority
    UnknownCa,
    /// Hostname mismatch
    CnMismatch,
    /// Expired certificate
    Expired,
    /// Not yet valid certificate
    NotYetValid,
    /// all other not separately classified certificate errors
    Other,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Diff,
    Patch,
}
