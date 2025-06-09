extern crate core;

mod args;
use args::AppArgs;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = AppArgs::parse();

    Ok(())
}
