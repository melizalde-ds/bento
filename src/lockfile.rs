use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display, path::PathBuf};

use crate::{
    config::{DependencyKey, DependencySpec, DependencyTable},
    resolver::Resolver,
};

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
        Resolver::package_verify(&self.0)
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

    pub fn _contains(&self, package: &str) -> bool {
        self.packages.contains_key(&LockfileKey::from(package))
    }

    pub fn sync(&mut self, configuration: &DependencyTable, add: bool) -> Result<()> {
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

        for (key, spec) in packages {
            let lockfile_key = to_lockfile_key(key, spec)?;
            let contains_key = self.packages.contains_key(&lockfile_key);
            if !contains_key && !add {
                bail!(
                    "Lockfile appears to be corrupted. Missing entry for package '{}'",
                    lockfile_key
                );
            } else if !contains_key {
                self.packages.insert(
                    lockfile_key,
                    LockfileEntry {
                        source: "unknown".to_string(),
                        checksum: "unknown".to_string(),
                        dependencies: vec![],
                    },
                );
            }
        }
        Ok(())
    }
}

fn to_lockfile_key(key: &DependencyKey, spec: &DependencySpec) -> Result<LockfileKey> {
    Ok(LockfileKey::from(format!("{}@{}", key, spec).as_str()))
}
