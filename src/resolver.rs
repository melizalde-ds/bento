use crate::config::DependencySpec;
use anyhow::Result;

const WIT_PACKAGE_REGEX: &str =
    r"^[a-zA-Z][a-zA-Z0-9_-]*:[a-zA-Z][a-zA-Z0-9_-]*@[0-9]+\.[0-9]+\.[0-9]+$";

impl DependencySpec {
    pub fn verify(&self) -> Result<bool> {
        let re = regex::Regex::new(WIT_PACKAGE_REGEX)?;
        match self {
            DependencySpec::Simple(a) => Ok(re.is_match(a)),
            DependencySpec::Detailed { version, features } => Ok(true),
        }
    }
}
