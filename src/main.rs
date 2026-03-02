use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    Add(Add),
    List,
    Remove,
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
        Commands::Add(args) => add(args),
        Commands::List => println!("Listing items"),
        Commands::Remove => println!("Removing item"),
    }
}

#[derive(Parser, Debug)]
#[command(about = "Add a new item")]
struct Add {
    name: String,
}

fn add(args: Add) {
    println!("Adding {}", args.name);
}
