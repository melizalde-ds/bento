use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    Add,
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
    println!("{:?}", cli);
}
