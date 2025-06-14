use clap::Parser;
use clap::Subcommand;

/// Subversion working copy identification tool.
///
///     Produce a compact version identifier for the working copy path
///     WC_PATH.  TRAIL_URL is the trailing portion of the URL used to
///     determine if WC_PATH itself is switched (detection of switches
///     within WC_PATH does not rely on TRAIL_URL).  The version identifier
///     is written to standard output.  For example:
///
///     $ svnversion . /repos/svn/trunk
///     4168
///
///   The version identifier will be a single number if the working
///   copy is single revision, unmodified, not switched and with
///   a URL that matches the TRAIL_URL argument.  If the working
///   copy is unusual the version identifier will be more complex:
///
///    4123:4168     mixed revision working copy
///    4168M         modified working copy
///    4123S         switched working copy
///    4123P         partial working copy, from a sparse checkout
///    4123:4168MS   mixed revision, modified, switched working copy
///
///   If WC_PATH is an unversioned path, the program will output
///   'Unversioned directory' or 'Unversioned file'.  If WC_PATH is
///   an added or copied or moved path, the program will output
///   'Uncommitted local addition, copy or move'.
///
///   If invoked without arguments WC_PATH will be the current directory.
#[derive(Parser, Debug)]
#[command(version, about, long_about, max_term_width = 80)]
pub struct AppArgs {
    /// do not output the trailing newline
    #[arg(short, long = "no-newline")]
    no_newline: bool,
    /// last changed rather than current revisions
    #[arg(short, long)]
    committed: bool,
    /// no progress (only errors) to stderr
    #[arg(short, long)]
    quiet: bool,

    #[arg(value_name = "WC_PATH", default_value = ".")]
    wc_path: String,
    #[arg(value_name = "TRAIL_URL")]
    trail_url: Option<String>,
}

impl AppArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        let local_abspath = "TODO:";

        let wc_ctx = svn_wc::Context::new();

        let res = wc_ctx.revision_status(local_abspath, &self.trail_url, self.committed)?;

        Ok(())
    }
}
