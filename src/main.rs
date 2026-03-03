mod cli;
mod commands;
mod config;
mod resolver;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};
use commands::{add, fetch, init, list, remove};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init(args) => init::run(args),
        Commands::Add(args) => add::run(args),
        Commands::Remove(args) => remove::run(args),
        Commands::Fetch(args) => fetch::run(args),
        Commands::List(args) => list::run(args),
    }
}
