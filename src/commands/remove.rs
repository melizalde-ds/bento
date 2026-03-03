use crate::cli;
use anyhow::Result;

pub fn remove(args: cli::Remove) -> Result<()> {
    println!("Removing {}", args.package);
    Ok(())
}
