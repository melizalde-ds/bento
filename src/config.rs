use std::{collections::BTreeMap, path::PathBuf};

use anyhow::bail;
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "bento.toml";

pub type DependencySection = BTreeMap<String, DependencySpec>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub project: Project,
    pub dependencies: Option<DependencySection>,
}

impl ProjectConfig {
    pub fn load() -> anyhow::Result<Self> {
        if !PathBuf::from("bento.toml").exists() {
            bail!(
                "Project not initialized in this directory. Please run `bento init` to create a new project."
            );
        }

        let content = std::fs::read_to_string(CONFIG_FILE)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let content = toml::to_string(self)?;
        std::fs::write(CONFIG_FILE, content)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
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

    // "wasi:http" = { version = "0.2.3", features = ["tls"] }
    Detailed {
        version: String,
        #[serde(default)]
        features: Vec<String>,
    },
}
