use sanity_client::SanityClient;
use clap::Parser;
use super::cli;

// #[cfg(feature = "sanrs")]
pub fn run() {
    let matches = ;
    match matches.command {
        cli::cli::Commands::Query => {
            let query_args = cli::cli::QueryArgs::parse();
            println!("{:?}", query_args);
        }
        cli::cli::Commands::Export => {
            println!("Exporting");
        }
    }
}





