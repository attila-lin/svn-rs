mod args;
mod context;
mod sub;

use args::AppArgs;
use args::SubCommand;

use clap::Parser;

fn main() {
    let cli = AppArgs::parse();

    match cli.command {
        SubCommand::Verify { repos_path } => {
            println!("Verifying repository at path: {}", repos_path);
            // Here you would call the function to verify the repository
            // For example: verify_repository(&repos_path);
        }
        _ => todo!("Other commands are not implemented yet"),
    }
}
