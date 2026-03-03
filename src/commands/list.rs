use crate::{cli, manifest::Manifest};
use anyhow::Result;

pub fn run(args: cli::List) -> Result<()> {
    let manifest = Manifest::load()?;
    let packages = manifest.get_packages();
    println!("Listing packages: {:?}", packages);
    Ok(())
}
