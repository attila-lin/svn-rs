use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about, max_term_width = 80)]
pub struct AppArgs {
    /// print nothing, or only summary information
    #[arg(short, long)]
    pub quiet: bool,
    /// descend recursively, same as --depth=infinity
    #[arg(short = 'R', long)]
    pub recursive: bool,
    /// obsolete; try --depth=files or --depth=immediates
    #[arg(short = 'N', long = "non-recursive")]
    pub non_recursive: bool,
    /// the change made by revision ARG (like -r ARG-1:ARG)
    /// If ARG is negative this is like -r ARG:ARG-1
    /// If ARG is of the form ARG1-ARG2 then this is like
    /// ARG1:ARG2, where ARG1 is inclusive
    #[arg(short, long)]
    pub change: Option<String>,
}
