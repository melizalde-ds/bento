use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

const LOCKFILE_NAME: &str = "bento.lock";

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord)]
struct LockfileKey(String);

#[derive(Debug, Deserialize, Serialize)]
pub struct Lockfile {
    pub packages: BTreeMap<LockfileKey, LockfileEntry>,
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

    pub fn contains(&self, package: &LockfileKey) -> bool {
        self.packages.contains_key(package)
    }
}
