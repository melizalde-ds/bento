use std::{collections::BTreeMap, fmt::Display, path::Path};

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

use crate::commands::ManifestResult;
use crate::package::Package;

const MANIFEST_FILE: &str = "bento.toml";

#[derive(Deserialize, Serialize)]
pub struct Manifest {
    pub project: ProjectMetadata,
    pub packages: BTreeMap<PackageKey, PackageSpec>,
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
        let packages = &self.packages;
        let mut result = Vec::new();
        for (key, spec) in packages {
            let package = Package::from_key_and_spec(key, spec)?;
            result.push(package);
        }
        Ok(result)
    }

    pub fn get_package(&self, key: &str) -> Result<Option<Package>> {
        let key = PackageKey(key.to_string());
        if let Some(spec) = self.packages.get(&key) {
            let package = Package::from_key_and_spec(&key, spec)?;
            Ok(Some(package))
        } else {
            Ok(None)
        }
    }

    pub fn add_packages<'a>(&'a mut self, packages: &'a [Package]) -> ManifestResult<'a> {
        let map = &mut self.packages;
        let mut added = vec![];
        let mut failed = vec![];

        for package in packages {
            match package.to_manifest_package() {
                Ok((key, spec)) => {
                    map.insert(key.clone(), spec);
                    added.push(package);
                }
                Err(e) => {
                    failed.push((package, e));
                }
            }
        }

        if failed.is_empty() {
            (added, None)
        } else {
            (added, Some(failed))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: String,
}

#[derive(Debug, Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Hash, Clone)]
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
