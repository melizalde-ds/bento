use std::{collections::BTreeMap, path::Path};

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

const MANIFEST_FILE: &str = "bento.toml";

#[derive(Deserialize, Serialize)]
pub struct Manifest {
    project: ProjectMetadata,
    packages: PackagesTable,
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
struct ProjectMetadata {
    name: String,
    version: String,
    description: Option<String>,
    author: String,
}

#[derive(Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq)]
struct PackagesTable {
    packages: Option<BTreeMap<PackageKey, PackageSpec>>,
}

#[derive(Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct PackageKey(String);

#[derive(Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum PackageSpec {
    Version(String),
    Features(PackageFeatures),
}

#[derive(Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct PackageFeatures {
    version: String,
    features: Vec<String>,
}
