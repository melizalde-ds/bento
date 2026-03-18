pub mod add;
pub mod fetch;
pub mod init;
pub mod list;
pub mod remove;

use std::collections::HashSet;

use crate::{lockfile::Lockfile, manifest::Manifest, package::Package};
use anyhow::Result;

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

fn flat_result<T, E>(results: Vec<Result<T, E>>) -> (HashSet<T>, HashSet<E>) {
    let mut oks: HashSet<T> = HashSet::new();
    let mut errs: HashSet<T> = HashSet::new();
    todo!("Implement a helper function to flatten Vec<Result<T, E>> into (HashSet<T>, HashSet<E>)")
}
