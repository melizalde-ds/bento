use crate::{lockfile::LockDetails, package::Package};
use anyhow::Result;

pub struct Resolver;
impl Resolver {
    pub fn _verify(package: &Package) -> Result<()> {
        package.verify()
    }

    pub fn lookup(dependency: &mut Package) -> Result<()> {
        if dependency.version == "latest" {
            dependency.version = "0.1.0".to_string();
        };
        Ok(())
    }

    pub fn resolve_packages(package: &Package) -> Result<(Package, LockDetails)> {
        Ok((
            package.clone(),
            LockDetails {
                checksum: "abc123".to_string(),
                source: "https://example.com/package.tar.gz".to_string(),
                dependencies: vec![],
            },
        ))
    }
}
