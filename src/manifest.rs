use std::collections::btree_map::Entry;
use std::{collections::BTreeMap, fmt::Display, path::Path};

use anyhow::{Result, anyhow, bail};
use serde::{Deserialize, Serialize};

use crate::commands::PackageResult;
use crate::package::Package;

const MANIFEST_FILE: &str = "bento.toml";

#[derive(Deserialize, Serialize, Debug)]
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
        let tmp = format!("{MANIFEST_FILE}.tmp");
        std::fs::write(&tmp, &content)?;
        std::fs::rename(&tmp, MANIFEST_FILE)?;
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

    pub fn add_packages<'a>(&'a mut self, packages: &'a [Package]) -> PackageResult<'a> {
        let map = &mut self.packages;
        let mut added = vec![];
        let mut failed = vec![];

        for package in packages {
            match package.to_manifest_package() {
                Ok((key, spec)) => {
                    match map.entry(key.clone()) {
                        Entry::Vacant(entry) => {
                            entry.insert(spec);
                            added.push(package);
                        }
                        Entry::Occupied(_) => {
                            failed.push((
                                package,
                                anyhow!("Package {key} already exists in manifest"),
                            ));
                        }
                    }
                }
                Err(e) => {
                    failed.push((
                        package,
                        anyhow!("Failed to convert package to manifest format: {e}"),
                    ));
                }
            }
        }

        if failed.is_empty() {
            (added, None)
        } else {
            (added, Some(failed))
        }
    }

    pub fn get_version(&self, key: &PackageKey) -> Option<&str> {
        self.packages.get(key).map(|spec| match spec {
            PackageSpec::Version(v) => v.as_str(),
        })
    }

    pub fn remove_package(&mut self, key: PackageKey) -> Result<(PackageKey, String)> {
        let map = &mut self.packages;

        match map.entry(key) {
            Entry::Occupied(entry) => {
                let version = match entry.get() {
                    PackageSpec::Version(v) => v.clone(),
                };
                let (key, _) = entry.remove_entry();
                Ok((key, version))
            }
            Entry::Vacant(entry) => bail!("Package {} not found in manifest", entry.into_key()),
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
