use clap::Parser;
use svnserve::AppArgs;

fn main() -> anyhow::Result<()> {
    let args = AppArgs::parse();

    Ok(())
}
