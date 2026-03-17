use anyhow::Result;

use crate::{
    cli,
    lockfile::{LockDetails, Lockfile},
    manifest::Manifest,
    package::Package,
    resolver::Resolver,
};

pub fn run(args: cli::Add) -> Result<()> {
    let mut manifest = Manifest::load()?;
    let mut lockfile = match Lockfile::load()? {
        Some(lockfile) => lockfile,
        None => Lockfile::create()?,
    };
    let packages = args
        .package
        .iter()
        .map(|p| Package::try_from(p.as_str()))
        .collect::<Result<Vec<Package>>>()?;

    let mut result = vec![];
    for mut package in packages {
        Resolver::lookup(&mut package)?;
        let resolved = Resolver::resolve_packages(&package)?;
        result.push(resolved);
    }

    add_packages(&mut manifest, &mut lockfile, result)?;
    manifest.save()?;
    lockfile.save()?;
    Ok(())
}

fn add_packages(
    manifest: &mut Manifest,
    lockfile: &mut Lockfile,
    packages_details: Vec<(Package, LockDetails)>,
) -> Result<()> {
    let packages = packages_details
        .iter()
        .map(|(package, _)| package)
        .cloned()
        .collect::<Vec<Package>>();
    manifest.add_packages(&packages)?;
    lockfile.add_packages(packages_details)?;
    let display: Vec<String> = packages.iter().map(|p| p.to_string()).collect();
    println!("Packages added successfully: {:?}", display);
    Ok(())
}
