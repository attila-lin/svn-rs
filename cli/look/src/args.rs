use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AppArgs {
    /// do not output the trailing newline
    #[arg(short, long)]
    no_newline: bool,
    /// last changed rather than current revisions
    #[arg(short, long)]
    committed: bool,
    /// no progress (only errors) to stderr
    #[arg(short, long)]
    quiet: bool,

    /// WC_PATH
    #[arg(value_name = "WC_PATH", default_value = ".")]
    wc_path: Option<String>,
    /// TRAIL_URL
    #[arg(value_name = "TRAIL_URL", default_value = "")]
    trail_url: String,
}
