use svn_repos::Repos;

use crate::args::AppArgs;

/// `subcommand_youngest`
pub fn run(repos_path: &str, args: &AppArgs) -> anyhow::Result<()> {
    let repos = Repos::open(repos_path)?;
    let youngest_rev = repos.youngest_rev()?;
    if args.no_newline {
        print!("{}", youngest_rev);
    } else {
        println!("{}", youngest_rev);
    }

    Ok(())
}
