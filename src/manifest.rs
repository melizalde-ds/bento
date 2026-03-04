use std::{collections::BTreeMap, fmt::Display, path::Path};

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

use crate::package::Package;

const MANIFEST_FILE: &str = "bento.toml";

#[derive(Deserialize, Serialize)]
pub struct Manifest {
    pub project: ProjectMetadata,
    pub packages: PackagesTable,
}

impl Manifest {
    pub fn load() -> Result<Self> {
        if !Path::new(MANIFEST_FILE).exists() {
            bail!(
                "Project not initialized in this directory. Please run `bento init` to create a new project."
            );
        }

        let content = std::fs::read_to_string(MANIFEST_FILE)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let content = toml::to_string(self)?;
        std::fs::write(MANIFEST_FILE, content)?;
        Ok(())
    }

    pub fn list_packages(&self) -> Result<Vec<Package>> {
        let Some(packages) = &self.packages.packages else {
            return Ok(vec![]);
        };
        let mut result = Vec::new();
        for (key, spec) in packages {
            let package = Package::from_key_and_spec(key, spec)?;
            result.push(package);
        }
        Ok(result)
    }

    pub fn get_package(&self, key: &str) -> Result<Option<Package>> {
        let key = PackageKey(key.to_string());
        let Some(packages) = &self.packages.packages else {
            return Ok(None);
        };
        if let Some(spec) = packages.get(&key) {
            let package = Package::from_key_and_spec(&key, spec)?;
            Ok(Some(package))
        } else {
            Ok(None)
        }
    }

    pub fn add_packages(&mut self, packages: &Vec<Package>) -> Result<()> {
        let map = self.packages.packages.get_or_insert_with(BTreeMap::new);
        for package in packages {
            let (key, spec) = package.to_manifest_package()?;
            map.insert(key, spec);
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: String,
}

#[derive(Debug, Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq)]
pub struct PackagesTable {
    pub packages: Option<BTreeMap<PackageKey, PackageSpec>>,
}

#[derive(Debug, Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PackageKey(pub String);

impl Display for PackageKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum PackageSpec {
    Version(String),
}
#[derive(Debug, Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct _PackageFeatures {
    pub version: String,
    pub features: Vec<String>,
}
