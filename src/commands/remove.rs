use crate::cli;
use crate::lockfile::{LockKey, Lockfile};
use crate::manifest::Manifest;
use crate::package::Package;
use anyhow::{Result, bail};

pub fn run(args: &cli::Remove) -> Result<()> {
    let (mut manifest, mut lockfile, packages) = super::load_packages(&args.package)?;

    remove_packages(&mut manifest, &mut lockfile, &packages)?;
    manifest.save()?;
    lockfile.save()?;
    Ok(())
}

fn remove_packages(
    manifest: &mut Manifest,
    lockfile: &mut Lockfile,
    packages: &[Package],
) -> Result<()> {
    let mut to_remove: Vec<(Package, LockKey)> = vec![];
    let mut errs = vec![];

    for package in packages {
        match package.to_manifest_package() {
            Ok((manifest_key, _)) => match manifest.get_version(&manifest_key) {
                Some(version) => {
                    let lock_key = LockKey::from_parts(&manifest_key, version);
                    if lockfile.packages.contains_key(&lock_key) {
                        to_remove.push((package.clone(), lock_key));
                    } else {
                        errs.push(format!("{package}: not found in lockfile"));
                    }
                }
                None => errs.push(format!("{package}: not found in manifest")),
            },
            Err(e) => errs.push(format!("{package}: {e}")),
        }
    }

    if !errs.is_empty() {
        bail!("Failed to remove packages:\n{}", errs.join("\n"));
    }

    let mut removed_names = vec![];
    for (package, lock_key) in &to_remove {
        let (manifest_key, _) = package.to_manifest_package()?;
        manifest.remove_package(manifest_key)?;
        lockfile.remove_package(lock_key.clone())?;
        removed_names.push(lock_key.0.as_str());
    }

    println!(
        "Packages removed successfully: {}",
        removed_names.join(", ")
    );
    Ok(())
}
