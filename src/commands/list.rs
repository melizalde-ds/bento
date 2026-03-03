use crate::{cli, manifest::Manifest};
use anyhow::{Ok, Result};

pub fn run(args: cli::List) -> Result<()> {
    let manifest = Manifest::load()?;
    match args.package {
        None => list_packages(&manifest),
        Some(packages) => {
            for package in packages {
                list_package(&manifest, &package)?;
            }
            Ok(())
        }
    }
}

fn list_packages(manifest: &Manifest) -> Result<()> {
    let packages = manifest.get_packages()?;
    if packages.is_empty() {
        println!("No packages found");
        return Ok(());
    }
    println!("Packages:");
    for package in packages {
        println!("- {}", package);
    }
    Ok(())
}

fn list_package(manifest: &Manifest, package_name: &str) -> Result<()> {
    let Some(package) = manifest.get_package(package_name)? else {
        println!("Package '{}' not found", package_name);
        return Ok(());
    };
    println!("{}", package);
    Ok(())
}
