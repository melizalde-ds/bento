use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init(Init),
    Add(Add),
    Remove(Remove),
    Fetch(Fetch),
    List(List),
}

#[derive(Parser, Debug)]
#[command(about = "Initialize a new project")]
pub struct Init {
    /// Project name; use '.' or '' to use the current directory name
    #[arg(
        value_name = "NAME",
        help = "Project name; use '.' or '' to use the current directory name"
    )]
    pub project: Option<String>,
}

#[derive(Parser, Debug)]
#[command(about = "Add a new WIT item")]
pub struct Add {
    /// Package in namespace:name@version format (e.g. wasi:http@0.2.3)
    #[arg(required = true, num_args = 1.., value_name = "PACKAGE")]
    pub package: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(about = "Remove a WIT item")]
pub struct Remove {
    /// Package in namespace:name@version format (e.g. wasi:http@0.2.3) or namespace:name format (e.g. wasi:http)
    #[arg(required = true, num_args = 1.., value_name = "PACKAGE")]
    pub package: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(about = "Fetch data from a source")]
pub struct Fetch {
    /// Force download even if the item already exists locally
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Parser, Debug)]
#[command(about = "List WIT items")]
pub struct List {
    /// Package in namespace:name@version format (e.g. wasi:http@0.2.3) or namespace:name format (e.g. wasi:http)
    #[arg(
        value_name = "PACKAGE",
        help = "Optional package name to filter the list"
    )]
    pub package: Option<Vec<String>>,
}
