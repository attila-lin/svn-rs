use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {}

fn main() {
    println!("Hello, world!");
}
