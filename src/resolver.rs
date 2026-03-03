use crate::package::Package;
use anyhow::Result;

pub struct Resolver;
impl Resolver {
    pub fn verify(package: &Package) -> Result<()> {
        package.verify()
    }

    pub fn lookup_package(dependency: &mut Package) -> Result<()> {
        if dependency.version == "latest" {
            dependency.version = "0.1.0".to_string();
        };
        Ok(())
    }
}
