use crate::{
    cli,
    manifest::{Manifest, ProjectMetadata},
};
use anyhow::{Result, bail};
use std::{collections::BTreeMap, path::Path};

pub fn run(args: &cli::Init) -> Result<()> {
    let project_name = match args.project.as_deref() {
        None | Some("." | "") => {
            let current_dir = std::env::current_dir()?;
            let dir_name = current_dir.file_name();
            match dir_name {
                Some(name) => name.to_string_lossy().to_string(),
                _ => bail!("Could not determine project name from current directory"),
            }
        }
        Some(name) => name.to_string(),
    };
    init_project(&project_name)
}

fn init_project(project: &str) -> Result<()> {
    if Path::new("bento.toml").exists() {
        bail!("Project already initialized in this directory");
    }
    let manifest = Manifest {
        project: ProjectMetadata {
            name: project.to_string(),
            version: "0.1.0".to_string(),
            description: None,
            author: "Your Name".to_string(),
        },
        packages: BTreeMap::new(),
    };
    manifest.save()?;
    println!("Initialized new project '{project}'");
    Ok(())
}
