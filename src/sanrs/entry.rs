#[cfg(feature = "sanrs")]
use crate::SanityClient;


#[cfg(feature = "sanrs")]
use clap::Parser;

#[cfg(feature = "sanrs")]
use crate::sanrs::cli;

#[cfg(feature = "sanrs")]
pub fn run() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Query(args) => {
            let dataset = args.dataset;
            let query = args.query;
            println!("Query args are {:?} and {:?}", dataset, query);
        },
        _ => {
            println!("No arguments entered");
        }
    }
}

