use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display, path::PathBuf};

use crate::config::{DependencyKey, DependencyTable};

const LOCKFILE_NAME: &str = "bento.lock";

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord)]
struct LockfileKey(String);

#[derive(Debug, Deserialize, Serialize)]
pub struct Lockfile {
    packages: BTreeMap<LockfileKey, LockfileEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LockfileEntry {
    pub source: String,
    pub checksum: String,
    pub dependencies: Vec<String>,
}

impl LockfileKey {
    pub fn verify(&self) -> Result<()> {
        let re = regex::Regex::new(
            r"^[a-zA-Z][a-zA-Z0-9_-]*:[a-zA-Z][a-zA-Z0-9_-]*@[0-9]+\.[0-9]+\.[0-9]+$",
        )?;
        if !re.is_match(&self.0) {
            anyhow::bail!("Invalid package format: {}", self.0);
        }
        Ok(())
    }
}

impl Display for LockfileKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for LockfileKey {
    fn from(value: &str) -> Self {
        LockfileKey(value.to_string())
    }
}

impl LockfileEntry {
    pub fn verify(&self) -> Result<()> {
        if self.source.is_empty() {
            bail!("Lockfile entry must have source");
        }
        Ok(())
    }
}

impl Lockfile {
    pub fn load() -> Result<Self> {
        if !PathBuf::from(&LOCKFILE_NAME).exists() {
            return Ok(Lockfile {
                packages: BTreeMap::new(),
            });
        };
        let content = std::fs::read_to_string(LOCKFILE_NAME)?;
        let lockfile: Self = toml::from_str(&content)?;
        lockfile.verify()?;
        Ok(lockfile)
    }

    pub fn save(&self) -> Result<()> {
        let content = toml::to_string(self)?;
        std::fs::write(LOCKFILE_NAME, content)?;
        Ok(())
    }

    fn verify(&self) -> Result<()> {
        for (name, entry) in &self.packages {
            name.verify()?;
            entry.verify()?;
        }
        Ok(())
    }

    pub fn contains(&self, package: &str) -> bool {
        self.packages.contains_key(&LockfileKey::from(package))
    }

    pub fn sync(&self, configuration: &DependencyTable) -> Result<()> {
        let packages = match &configuration.packages {
            None => {
                if self.packages.is_empty() {
                    return Ok(());
                } else {
                    bail!(
                        "Configuration has no packages but lockfile is not empty. Lockfile was edited manually or is corrupted."
                    );
                }
            }
            Some(packages) => packages,
        };
        for package in packages.keys() {
            let lockfile_key = LockfileKey::from(package);
            if !self.packages.contains_key(&lockfile_key) {
                bail!(
                    "Lockfile is missing package '{}' specified in configuration. Lockfile is out of sync with configuration.",
                    package
                );
            }
        }
        Ok(())
    }
}

impl From<&DependencyKey> for LockfileKey {
    fn from(value: &DependencyKey) -> Self {
        let string = value.to_string();
        let split = string.split(" = ").collect::<Vec<&str>>();
        if split.len() != 2 {
            panic!("Invalid dependency key format: {}", string);
        }
        LockfileKey(format!("{}@{}", split[0], split[1]))
    }
}
