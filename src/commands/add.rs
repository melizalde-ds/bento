use crate::cli;
use anyhow::Result;

pub fn run(args: cli::Add) -> Result<()> {
    println!("Adding {}", args.package);
    Ok(())
}
