use crate::lockfile::Lockfile;
use crate::{cli, config::Manifest, resolver::Resolver};
use anyhow::Result;

pub fn run(args: cli::Add) -> Result<()> {
    let packages = args.package;
    for package in packages {
        Resolver::verify(&package)?;
        println!("Adding package: {}", package);
    }
    let _config = Manifest::load()?;
    let _lockfile = Lockfile::load()?;
    Ok(())
}
