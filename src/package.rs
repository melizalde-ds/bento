use std::fmt::Display;

use anyhow::{Result, bail};

use crate::manifest::{PackageKey, PackageSpec};

#[derive(Debug, Clone)]
pub struct Package {
    pub namespace: String,
    pub name: String,
    pub version: String,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}@{}", self.namespace, self.name, self.version)
    }
}

impl TryFrom<&str> for Package {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let dependency = package_str_verify(value)?;
        Ok(dependency)
    }
}

impl Package {
    pub fn verify(&self) -> Result<()> {
        if self.namespace.is_empty() || self.name.is_empty() || self.version.is_empty() {
            bail!("Package namespace, name, and version cannot be empty");
        }
        Ok(())
    }

    pub fn extract(&self) -> (String, String, String) {
        (
            self.namespace.clone(),
            self.name.clone(),
            self.version.clone(),
        )
    }

    pub fn from_key_and_spec(key: &PackageKey, spec: &PackageSpec) -> Result<Self> {
        let mut package = namespace_and_name(key.to_string().as_str())?;
        package.version = match spec {
            PackageSpec::Version(version) => version.clone(),
        };
        Ok(package)
    }

    pub fn to_manifest_package(&self) -> Result<(PackageKey, PackageSpec)> {
        self.verify()?;
        let (namespace, name, version) = self.extract();
        let key = PackageKey(format!("{namespace}:{name}"));
        let spec = PackageSpec::Version(version);
        Ok((key, spec))
    }
}

fn package_str_verify(package: &str) -> Result<Package> {
    let dependency = if package.contains('@') {
        full_package_name(package)?
    } else {
        namespace_and_name(package)?
    };
    dependency.verify()?;
    Ok(dependency)
}

fn full_package_name(package: &str) -> Result<Package> {
    let parts = package.split('@').collect::<Vec<&str>>();
    if parts.len() != 2 {
        bail!("Package '{package}' must be in 'namespace:name@version' format");
    }
    let mut dependency = namespace_and_name(parts[0])?;
    dependency.version = parts[1].to_string();
    Ok(dependency)
}

fn namespace_and_name(package: &str) -> Result<Package> {
    let parts = package.split(':').collect::<Vec<&str>>();
    if parts.len() != 2 {
        bail!(
            "Package '{package}' must be in 'namespace:name' format when version is not specified"
        );
    }
    Ok(Package {
        namespace: parts[0].to_string(),
        name: parts[1].to_string(),
        version: "latest".to_string(),
    })
}
