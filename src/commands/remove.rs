use crate::cli;
use anyhow::Result;

pub fn run(args: &cli::Remove) -> Result<()> {
    let (_manifest, _lockfile, _packages) = super::load_packages(&args.package)?;

    Ok(())
}
