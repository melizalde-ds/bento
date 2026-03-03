use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::{collections::BTreeMap, path::PathBuf};

const MANIFEST_FILE: &str = "bento.toml";

pub type DependencySection = BTreeMap<DependencyKey, DependencySpec>;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyKey(String);

impl Display for DependencyKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for DependencyKey {
    fn from(value: &str) -> Self {
        DependencyKey(value.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub project: ProjectMetadata,
    pub dependencies: DependencyTable,
}

impl Manifest {
    pub fn load() -> Result<Self> {
        if !PathBuf::from(MANIFEST_FILE).exists() {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct DependencyTable {
    pub packages: Option<DependencySection>,
}

impl DependencyTable {
    pub fn add_package(&mut self, key: DependencyKey, spec: DependencySpec) -> Result<()> {
        if self.packages.is_none() {
            self.packages = Some(BTreeMap::new());
        }
        let packages = self.packages.as_mut();
        match packages {
            None => unreachable!(),
            Some(map) => {
                map.insert(key, spec);
            }
        };
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

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum DependencySpec {
    // "wasi:http" = "0.2.3"
    Simple(String),
}
