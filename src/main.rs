use std::path::PathBuf;

use anyhow::{Ok, Result, bail};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Subcommand, Debug)]
enum Commands {
    Init(Init),
    Add(Add),
    Remove(Remove),
    Fetch(Fetch),
    List(List),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

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

#[derive(Parser, Debug)]
#[command(about = "Initialize a new project")]
struct Init {
    project: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProjectConfig {
    name: String,
    authors: Vec<String>,
    version: String,
    description: Option<String>,
    workspace: Vec<PathBuf>,
    dependencies: Option<DependencyConfig>,
}

#[derive(Debug, Deserialize, Serialize)]

struct DependencyConfig {
    dev: Option<Vec<Dependency>>,
    dependencies: Option<Vec<Dependency>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Dependency {
    name: String,
    version: String,
}

#[derive(Parser, Debug)]
#[command(about = "Add a new WIT item")]
struct Add {
    /// Package in namespace:name@version format (e.g. wasi:http@0.2.3)
    package: String,
}

#[derive(Parser, Debug)]
#[command(about = "Remove a WIT item")]
struct Remove {
    package: String,
}

#[derive(Parser, Debug)]
#[command(about = "Fetch data from a source")]
struct Fetch;

#[derive(Parser, Debug)]
#[command(about = "List WIT items")]
struct List {
    #[arg(short, long)]
    all: bool,

    package: Option<String>,
}

fn init(args: Init) -> Result<()> {
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
    let content = toml::to_string(&ProjectConfig {
        name: project.to_string(),
        authors: vec!["".to_string()],
        version: "0.1.0".to_string(),
        description: None,
        workspace: vec![],
        dependencies: None,
    })?;
    std::fs::write("bento.toml", content)?;
    println!("Initialized project '{}'", project);
    Ok(())
}

fn add(args: Add) -> Result<()> {
    println!("Adding {}", args.package);
    Ok(())
}

fn remove(args: Remove) -> Result<()> {
    println!("Removing {}", args.package);
    Ok(())
}

fn fetch(_args: Fetch) -> Result<()> {
    println!("Fetching data");
    Ok(())
}

fn list(args: List) -> Result<()> {
    println!("Listing WIT items (all = {})", args.all);
    Ok(())
}
