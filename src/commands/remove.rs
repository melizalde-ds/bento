use crate::lockfile::Lockfile;
use crate::manifest::Manifest;
use crate::{cli, lockfile, manifest};
use anyhow::Result;

pub fn run(args: &cli::Remove) -> Result<()> {
    let (mut manifest, mut lockfile, packages) = super::load_packages(&args.package)?;
    remove_packages(&mut manifest, &mut lockfile);
    Ok(())
}

fn remove_packages(manifest: &mut Manifest, lockfile: &mut Lockfile) {}
