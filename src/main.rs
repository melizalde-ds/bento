mod cli;
mod commands;
mod config;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};
use commands::add::add;
use commands::fetch::fetch;
use commands::init::init;
use commands::list::list;
use commands::remove::remove;

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
