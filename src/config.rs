use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub project: Project,
    pub dependencies: DependencyConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DependencyConfig {
    pub dependencies: Option<BTreeMap<String, DependencySpec>>,
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
