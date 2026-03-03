use std::fmt::Display;

use anyhow::{Result, bail};

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

    pub fn extract(&self) -> Result<(String, String, String)> {
        return Ok((
            self.namespace.clone(),
            self.name.clone(),
            self.version.clone(),
        ));
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
        bail!(
            "Package '{}' must be in 'namespace:name@version' format",
            package
        );
    };
    let mut dependency = namespace_and_name(parts[0])?;
    dependency.version = parts[1].to_string();
    Ok(dependency)
}

fn namespace_and_name(package: &str) -> Result<Package> {
    let parts = package.split(':').collect::<Vec<&str>>();
    if parts.len() != 2 {
        bail!(
            "Package '{}' must be in 'namespace:name' format when version is not specified",
            package
        );
    }
    Ok(Package {
        namespace: parts[0].to_string(),
        name: parts[1].to_string(),
        version: String::new(),
    })
}
