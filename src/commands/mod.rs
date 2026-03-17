pub mod add;
pub mod fetch;
pub mod init;
pub mod list;
pub mod remove;

use anyhow::Result;

use crate::{lockfile::Lockfile, manifest::Manifest, package::Package};

fn load_packages(package_args: &[String]) -> Result<(Manifest, Lockfile, Vec<Package>)> {
    let manifest = Manifest::load()?;
    let lockfile = match Lockfile::load()? {
        Some(lockfile) => lockfile,
        None => Lockfile::create()?,
    };
    let packages = package_args
        .iter()
        .map(|p| Package::try_from(p.as_str()))
        .collect::<Result<Vec<Package>>>()?;
    Ok((manifest, lockfile, packages))
}
