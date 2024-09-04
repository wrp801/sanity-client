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
    // Export(ExportArgs),
}

#[derive(Parser, Debug)]
pub struct QueryArgs {
    #[clap(short, long, required = false)]
    pub token: Option<String>,

    #[clap(short='d', long, required = true)]
    pub dataset: String,

    #[clap(short='q', long, required = true)]
    pub query: String, 

    #[clap(short='i', long, required = false, action)]
    pub interactive: Option<bool>,
}

