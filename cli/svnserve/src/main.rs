use clap::Parser;
use svnserve::AppArgs;
use tracing::info;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv().ok();

    info!("svnserve start");

    let args = AppArgs::parse();

    Ok(())
}
