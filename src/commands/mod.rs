pub mod add;
pub mod fetch;
pub mod init;
pub mod list;
pub mod remove;

use std::collections::{HashMap, HashSet};

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

fn _flat_result<T, E>(_results: ManifestResult) -> (HashSet<T>, HashMap<T, E>) {
    let mut _oks: HashSet<T> = HashSet::new();
    let mut _errs: HashMap<T, E> = HashMap::new();
    todo!("Implement a helper function to flatten Vec<Result<T, E>> into (HashSet<T>, HashSet<E>)")
}

pub type ManifestResult<'a> = (Vec<&'a Package>, Option<Vec<(&'a Package, Error)>>);

pub type LockfileResult<'a> = (Vec<&'a Package>, Option<Vec<(&'a Package, Error)>>);
