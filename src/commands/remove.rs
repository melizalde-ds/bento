use crate::{cli, package::Package};
use anyhow::Result;

pub fn run(args: cli::Remove) -> Result<()> {
    let mut manifest = crate::manifest::Manifest::load()?;
    let mut lockfile = match crate::lockfile::Lockfile::load()? {
        Some(lockfile) => lockfile,
        None => crate::lockfile::Lockfile::create()?,
    };

    let packages = args
        .package
        .iter()
        .map(|p| Package::try_from(p.as_str()))
        .collect::<Result<Vec<Package>>>()?;

    Ok(())
}
