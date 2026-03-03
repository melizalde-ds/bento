use crate::cli;
use anyhow::Result;

pub fn list(args: cli::List) -> Result<()> {
    println!("Listing WIT items (all = {})", args.all);
    Ok(())
}
