use anyhow::{Result, bail};
use regex::Regex;

const WIT_PACKAGE_REGEX: &str =
    r"^[a-zA-Z][a-zA-Z0-9_-]*:[a-zA-Z][a-zA-Z0-9_-]*@[0-9]+\.[0-9]+\.[0-9]+$";

pub struct Resolver;
impl Resolver {
    pub fn package_verify(package: &str) -> Result<()> {
        let re = Regex::new(WIT_PACKAGE_REGEX)?;
        if !re.is_match(package) {
            bail!("Invalid package format: {}", package);
        }
        Ok(())
    }

    pub fn to_dependency(package: &str) -> Result<(String, String, String)> {
        let parts = package.split('@').collect::<Vec<&str>>();
        let name_parts = parts[0].split(':').collect::<Vec<&str>>();
        Ok((
            name_parts[0].to_string(),
            name_parts[1].to_string(),
            parts[1].to_string(),
        ))
    }
}
