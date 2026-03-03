use anyhow::Result;

use crate::{cli, lockfile::Lockfile, manifest::Manifest, package, resolver::Resolver};

pub fn run(_args: cli::Add) -> Result<()> {
    let manifest = Manifest::load()?;
    let Some(mut lockfile) = Lockfile::load()? else {
        println!("No lockfile found. Please run `bento fetch` to generate a lockfile.");
        return Ok(());
    };

    // TODO: Implement comparison between manifest and lockfile, and update lockfile with new packages from manifest

    manifest.save()?;
    lockfile.save()?;
    Ok(())
}
