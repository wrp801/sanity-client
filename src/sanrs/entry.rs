// #[cfg(feature = "sanrs")]
use {
    crate::sanity::client::SanityClient,
    clap::Parser,
    crate::sanrs::{cli, query_shell, config},
    std::fs::File,
    std::io::prelude::*,
    std::fs,
    std::env,
    std::path::Path,
    std::process
};

fn config_exists() -> bool {
    let home = env::var("HOME").expect("Home is not properly defined");
    let file_path = Path::new(&home).join(".sanity/.sanityrc");

    let res =  std::fs::metadata(file_path);

    match res {
        Ok(res) => true,
        Err(res) => false
    }
}


// TODO: fill out docs
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {

    if !config_exists() {
        eprintln!("Config file does not exist. Run `sanrs init` to create a config file");
        process::exit(1);
    }
    let config = config::read_config()?;
    let token = config.env.api_token;
    let dataset = config.env.dataset;
    let project = config.env.project_id;

    let client = SanityClient::new(token, dataset, project);


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
        cli::Commands::Config(cmds) => match cmds {
            cli::ConfigCmds::Init(args) => {
                let name = args.name;
                let token = args.token;
                let project = args.project;
                let dataset = args.dataset;
                let toml = config::create_env_toml(name, token, dataset, project);

                let _ = config::create_file(&toml);
                println!("Successfully created .sanityrc");
            }

            cli::ConfigCmds::Add(args) => {
                println!("Entered the add zone")
            }
            cli::ConfigCmds::Remove(args) => {
                let retain = args.retain;
                
                if retain {
                    /// wipe the contents of the file

                    todo!();
                } else {
                    /// delete the .sanityrc file

                    let home = env::var("HOME").expect("HOME env variable not set");
                    let path = Path::new(&home).join(".sanity/.sanityrc");

                   let _ = fs::remove_file(path);
                    println!("Successfully removed file")
                }
            }
            _ => {
                println!("Entered unknown territory")
            }
        },
        _ => {
            println!("No arguments entered");
        }
    }
    Ok(())
}

pub fn main() {}
