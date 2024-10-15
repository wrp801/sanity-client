#[cfg(feature = "sanrs")]
use clap::{Subcommand, Parser};

#[derive(Parser, Debug)]
#[clap(name = "sanrs", version = "0.1.0", about = "A simple CLI for sanity")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Query(QueryArgs),
    #[command(subcommand)]
    Config(ConfigCmds),
}

#[derive(Debug, Subcommand)]
pub enum ConfigCmds {
    /// Initialize a new config file
    Init(ConfigArgs),
    /// Add a new section to the config file
    Add(ConfigArgs),
    Remove(RemoveArgs),
    View
}

#[derive(Parser, Debug)]
pub struct QueryArgs {
    /// The value of the API token to access Sanity
    #[clap(short, long, required = false)]
    pub token: Option<String>,

    /// The name of the Sanity Dataset
    #[clap(short = 'd', long, required = true)]
    pub dataset: String,

    /// The optional GROQ query to execute. Either a query or the `-i` flag need to be passed
    #[clap(short = 'q', long, required = false)]
    pub query: Option<String>, 

    /// Determines whether sanrs will enter interactive mode. Interactive mode allows for continual
    /// execution of GROQ queries against the Sanity API, each time a GROQ query is entered, the
    /// results will be printed to the console
    #[clap(short = 'i', long, required = false, action = clap::ArgAction::SetTrue)]
    pub interactive: bool,
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    /// The name of the configured group. This allows for the ability to have multiple environments
    /// with different datasets/projects/tokens defined in each
    #[clap(short, long, required = true)]
    pub name: String,

    /// The value of the API token to access Sanity
    #[clap(short, long, required = true)]
    pub token: String,

    /// The name of the Sanity Dataset
    #[clap(short, long, required = true)]
    pub dataset: String,

    /// The project of the associated token and dataset
    #[clap(short, long, required = true)]
    pub project: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    #[clap(short, long, required = false, action = clap::ArgAction::SetTrue)]
    pub retain: bool
}
