use crate::config::{DependencyKey, DependencySpec};
use crate::lockfile::Lockfile;
use crate::resolver::Package;
use crate::{cli, config::Manifest, resolver::Resolver};
use anyhow::{Result, anyhow};

pub fn run(args: cli::Add) -> Result<()> {
    let packages = args.package;
    let packages: Result<Vec<Package>> = packages
        .iter()
        .map(|d| {
            let dependency = Resolver::package_verify(d)
                .map_err(|e| anyhow!("Failed to verify package '{}': {}", d, e));
            dependency.and_then(|mut d| {
                let result = Resolver::lookup_package(&mut d);
                match result {
                    Ok(_) => Ok(d),
                    Err(e) => Err(anyhow!("Failed to lookup package '{}': {}", d, e)),
                }
            })
        })
        .collect();
    let packages = packages?;

    let mut config = Manifest::load()?;

    for package in packages {
        let key = DependencyKey::from(package.to_string().as_str());
        let spec = DependencySpec::from(package.to_string().as_str());
        config.dependencies.add_package(key, spec)?;
    }
    let mut lockfile = Lockfile::load()?;
    lockfile.sync(&config.dependencies, true)?;

    config.save()?;
    lockfile.save()?;
    println!("Packages added successfully.");
    Ok(())
}
