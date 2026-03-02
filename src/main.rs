use std::path::PathBuf;

use clap::{Parser, Subcommand};

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

fn main() {
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
    name: String,
    #[arg(short, long, default_value = "")]
    output: PathBuf,
}

#[derive(Parser, Debug)]
#[command(about = "Add a new WIT item")]
struct Add {
    name: String,
}

#[derive(Parser, Debug)]
#[command(about = "Remove a WIT item")]
struct Remove {
    name: String,
}

#[derive(Parser, Debug)]
#[command(about = "Fetch data from a source")]
struct Fetch;

#[derive(Parser, Debug)]
#[command(about = "List WIT items")]
struct List {
    #[arg(short, long, default_value_t = true)]
    all: bool,
}

fn init(args: Init) {
    println!(
        "Initializing project {} in {}",
        args.name,
        args.output.display()
    );
}

fn add(args: Add) {
    println!("Adding {}", args.name);
}

fn remove(args: Remove) {
    println!("Removing {}", args.name);
}

fn fetch(_args: Fetch) {
    println!("Fetching data");
}

fn list(args: List) {
    println!("Listing WIT items (all = {})", args.all);
}
