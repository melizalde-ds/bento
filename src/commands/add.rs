use crate::config::{DependencyKey, DependencySpec};
use crate::lockfile::Lockfile;
use crate::{cli, config::Manifest, resolver::Resolver};
use anyhow::Result;

pub fn run(args: cli::Add) -> Result<()> {
    let packages = args.package;
    for package in &packages {
        Resolver::package_verify(package)?;
    }
    let mut config = Manifest::load()?;
    for package in packages {
        let key = DependencyKey::from(package.as_str());
        let spec = DependencySpec::from(package.as_str());
        config.dependencies.add_package(key, spec)?;
    }
    let mut lockfile = Lockfile::load()?;
    lockfile.sync(&config.dependencies, true)?;

    config.save()?;
    lockfile.save()?;
    Ok(())
}
