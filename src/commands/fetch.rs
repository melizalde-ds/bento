use crate::cli;
use anyhow::Result;

pub fn run(_args: cli::Fetch) -> Result<()> {
    println!("Fetching data");
    Ok(())
}
