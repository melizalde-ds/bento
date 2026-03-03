use crate::cli;
use crate::config;
use anyhow::Result;

pub fn run(args: cli::List) -> Result<()> {
    let config = config::ProjectConfig::load()?;
    println!(
        "Project: {} v{}",
        config.project.name, config.project.version
    );
    Ok(())
}
