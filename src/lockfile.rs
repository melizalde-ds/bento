use std::{collections::BTreeMap, fmt::Display, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::package::Package;

const LOCKFILE_NAME: &str = "bento.lock";

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
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

    pub fn create() -> Result<Lockfile> {
        let lockfile = Lockfile {
            packages: BTreeMap::new(),
        };
        lockfile.save()?;
        Ok(lockfile)
    }

    pub fn add_packages(&mut self, packages: Vec<(Package, LockDetails)>) -> Result<()> {
        let packages = packages
            .into_iter()
            .map(|(package, details)| {
                let key = LockKey(package.to_string());
                (key, details)
            })
            .collect::<Vec<(LockKey, LockDetails)>>();

        self.packages.extend(packages);
        Ok(())
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LockKey(pub String);

impl Display for LockKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LockDetails {
    pub source: String,
    pub checksum: String,
    pub dependencies: Vec<String>,
}
