use std::collections::btree_map::Entry;
use std::vec;
use std::{collections::BTreeMap, fmt::Display, path::Path};

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::commands::LockfileResult;
use crate::package::Package;

const LOCKFILE_NAME: &str = "bento.lock";

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct Lockfile {
    pub packages: BTreeMap<LockKey, LockDetails>,
    pub dependencies: BTreeMap<LockKey, Vec<LockKey>>,
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
            dependencies: BTreeMap::new(),
        };
        lockfile.save()?;
        Ok(lockfile)
    }

    pub fn add_packages<'a>(
        &'a mut self,
        packages: &'a [(Package, LockDetails)],
    ) -> LockfileResult<'a> {
        let mut oks = vec![];
        let mut errs = vec![];
        for (package, details) in packages {
            let key = LockKey(package.to_string());
            match self.packages.entry(key.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(details.clone());
                    oks.push(package);
                }
                Entry::Occupied(_) => {
                    errs.push((package, anyhow!("Package {key} already exists in lockfile")));
                }
            }
            for dep in &details.dependencies {
                match self.dependencies.entry(LockKey(dep.clone())) {
                    Entry::Occupied(entry) => {
                        let deps = entry.into_mut();
                        if !deps.contains(&key) {
                            deps.push(key.clone());
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![key.clone()]);
                    }
                }
            }
        }

        if errs.is_empty() {
            (oks, None)
        } else {
            (vec![], Some(errs))
        }
    }

    pub fn _remove_packages(&mut self, packages: Vec<Package>) -> Vec<Result<LockKey>> {
        let package_keys = packages
            .into_iter()
            .map(|package| LockKey(package.to_string()))
            .collect::<Vec<LockKey>>();

        let mut results = Vec::new();
        for key in package_keys {
            if self.packages.remove(&key).is_some() {
                results.push(Ok(key));
            } else {
                results.push(Err(anyhow::anyhow!("Package {key} not found in lockfile")));
            }
        }
        results
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct LockKey(pub String);

impl Display for LockKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct LockDetails {
    pub source: String,
    pub checksum: String,
    pub dependencies: Vec<String>,
}
