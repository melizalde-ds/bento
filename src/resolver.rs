use anyhow::{Result, bail};
use regex::Regex;

const WIT_PACKAGE_REGEX: &str =
    r"^[a-zA-Z][a-zA-Z0-9_-]*:[a-zA-Z][a-zA-Z0-9_-]*@[0-9]+\.[0-9]+\.[0-9]+$";

pub struct Resolver;

impl Resolver {
    pub fn verify(package: &String) -> Result<()> {
        let re = Regex::new(WIT_PACKAGE_REGEX)?;
        if !re.is_match(package) {
            bail!("Invalid package format: {}", package);
        }
        Ok(())
    }
}
