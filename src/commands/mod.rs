pub mod add;
pub mod fetch;
pub mod init;
pub mod list;
pub mod remove;

use crate::manifest::Manifest;
use crate::{lockfile::Lockfile, package::Package};
use anyhow::{Error, Result};

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

pub type PackageResult<'a> = (Vec<&'a Package>, Option<Vec<(&'a Package, Error)>>);
