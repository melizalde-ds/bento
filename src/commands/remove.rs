use crate::cli;
use crate::lockfile::Lockfile;
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

fn remove_packages(manifest: &mut Manifest, lockfile: &mut Lockfile, packages: &[Package]) -> Result<()> {
    let mut removed = vec![];
    let mut errs = vec![];
    for package in packages {
        match package.to_manifest_package() {
            Ok((key, _)) => match manifest.remove_package(key) {
                Ok(key) => match lockfile.remove_package(key) {
                    Ok(key) => removed.push(key),
                    Err(e) => errs.push(format!("{package}: {e}")),
                },
                Err(e) => errs.push(format!("{package}: {e}")),
            },
            Err(e) => {
                errs.push(format!("{package}: {e}"));
            }
        }
    }

    if !errs.is_empty() {
        bail!("Failed to remove packages:\n{}", errs.join("\n"));
    }

    println!(
        "Packages removed successfully: {}",
        removed
            .iter()
            .map(|s| s.0.as_str())
            .collect::<Vec<&str>>()
            .join(", ")
    );
    Ok(())
}
