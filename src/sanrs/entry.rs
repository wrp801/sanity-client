#[cfg(feature = "sanrs")]
use crate::sanity::client::SanityClient;

#[cfg(feature = "sanrs")]
use clap::Parser;

#[cfg(feature = "sanrs")]
use crate::sanrs::cli;

#[cfg(feature = "sanrs")]
use crate::sanrs::query_shell;

#[cfg(feature = "sanrs")]
pub async fn run(client: &SanityClient) {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Query(args) => {
            let interactive = args.interactive;
            if interactive {
                println!("Entering interactive query shell");
                // spin up the query shell
                let client_clone = client.clone();
                let _ = query_shell::run_shell(&client_clone).await;
            } else {
                let dataset = args.dataset;
                let query = args.query;
            }
        }
        _ => {
            println!("No arguments entered");
        }
    }
}

pub fn main() {}
