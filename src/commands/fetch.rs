use crate::cli;
use anyhow::Result;

pub fn fetch(_args: cli::Fetch) -> Result<()> {
    println!("Fetching data");
    Ok(())
}
