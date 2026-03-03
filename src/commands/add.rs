use crate::cli;
use anyhow::Result;

pub fn add(args: cli::Add) -> Result<()> {
    println!("Adding {}", args.package);
    Ok(())
}
