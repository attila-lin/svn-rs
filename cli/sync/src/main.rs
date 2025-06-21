mod baton;

mod args;
use args::AppArgs;

mod sub;

use clap::Parser;
fn main() -> anyhow::Result<()> {
    let _args = AppArgs::parse();
    Ok(())
}
