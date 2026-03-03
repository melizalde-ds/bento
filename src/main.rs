mod cli;
mod config;

use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::Parser;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init(args) => init(args),
        Commands::Add(args) => add(args),
        Commands::Remove(args) => remove(args),
        Commands::Fetch(args) => fetch(args),
        Commands::List(args) => list(args),
    }
}

fn init(args: cli::Init) -> Result<()> {
    let project_name = match args.project.as_deref() {
        None | Some(".") => {
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
    if PathBuf::from("bento.toml").exists() {
        bail!("Project already initialized in this directory");
    }

    let content = toml::to_string(&config::ProjectConfig {
        project: config::Project {
            name: project.to_string(),
            version: "0.1.0".to_string(),
            description: None,
            author: "Author Name".to_string(),
        },
        dependencies: config::DependencyConfig { dependencies: None },
    })?;

    std::fs::write("bento.toml", &content)?;
    println!("Initialized new project:\n{}", content);
    Ok(())
}

fn add(args: cli::Add) -> Result<()> {
    println!("Adding {}", args.package);
    Ok(())
}

fn remove(args: cli::Remove) -> Result<()> {
    println!("Removing {}", args.package);
    Ok(())
}

fn fetch(_args: cli::Fetch) -> Result<()> {
    println!("Fetching data");
    Ok(())
}

fn list(args: cli::List) -> Result<()> {
    println!("Listing WIT items (all = {})", args.all);
    Ok(())
}
