mod args;
use args::AppArgs;

use clap::Parser;
fn main() -> anyhow::Result<()> {
    let _args = AppArgs::parse();
    Ok(())
}
