use crate::cli;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;
use anyhow::Result;

pub fn run(args: &cli::Remove) -> Result<()> {
    let (mut manifest, mut lockfile, _packages) = super::load_packages(&args.package)?;
    remove_packages(&mut manifest, &mut lockfile);
    Ok(())
}

fn remove_packages(_manifest: &mut Manifest, _lockfile: &mut Lockfile) {}
