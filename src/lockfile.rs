use std::{collections::BTreeMap, fmt::Display, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::package::Package;

const LOCKFILE_NAME: &str = "bento.lock";

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lockfile {
    pub packages: BTreeMap<LockKey, LockDetails>,
}

impl Lockfile {
    pub fn load() -> Result<Option<Self>> {
        if !Path::new(LOCKFILE_NAME).exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(LOCKFILE_NAME)?;
        let lockfile: Self = toml::from_str(&content)?;
        Ok(Some(lockfile))
    }

    pub fn save(&self) -> Result<()> {
        let content = toml::to_string(self)?;
        std::fs::write(LOCKFILE_NAME, content)?;
        Ok(())
    }

    pub fn _list_packages(&self) -> Result<Vec<Package>> {
        let mut result = Vec::new();
        for key in self.packages.keys() {
            let package = Package::from_lock_key(key)?;
            result.push(package);
        }
        Ok(result)
    }

    pub fn _get_package(&self, key: &str) -> Result<Option<Package>> {
        let key = LockKey(key.to_string());
        if self.packages.contains_key(&key) {
            let package = Package::from_lock_key(&key)?;
            Ok(Some(package))
        } else {
            Ok(None)
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LockKey(pub String);

impl Display for LockKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LockDetails {
    pub source: String,
    pub checksum: String,
    pub dependencies: Vec<String>,
}
