use crate::cli;
use anyhow::Result;

pub fn run(args: cli::Remove) -> Result<()> {
    println!("Removing {}", args.package);
    Ok(())
}
