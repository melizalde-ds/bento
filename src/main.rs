mod cli;
mod commands;
mod config;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};
use commands::init::init;

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
