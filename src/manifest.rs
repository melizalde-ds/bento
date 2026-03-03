use std::{collections::BTreeMap, path::Path};

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

const MANIFEST_FILE: &str = "bento.toml";

#[derive(Deserialize, Serialize)]
pub struct Manifest {
    pub project: ProjectMetadata,
    pub packages: Option<PackagesTable>,
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
}

#[derive(Deserialize, Serialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: String,
}

#[derive(Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq)]
pub struct PackagesTable {
    pub packages: Option<BTreeMap<PackageKey, PackageSpec>>,
}

#[derive(Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PackageKey(pub String);

#[derive(Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PackageSpec {
    Version(String),
    Features(PackageFeatures),
}

#[derive(Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PackageFeatures {
    pub version: String,
    pub features: Vec<String>,
}
