//!  Subversion server inspection tool main file.

mod args;
use args::AppArgs;

mod sub;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = AppArgs::parse();

    if let Err(e) = args.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
