use std::vec;

use crate::lockfile::Lockfile;
use crate::{cli, config::Manifest, resolver::Resolver};
use anyhow::Result;

pub fn run(args: cli::Add) -> Result<()> {
    let packages = args.package;
    for package in &packages {
        Resolver::verify(package)?;
    }
    let config = Manifest::load()?;
    let mut lockfile = Lockfile::load()?;
    lockfile.sync(&config.dependencies)?;
    todo!();
}
