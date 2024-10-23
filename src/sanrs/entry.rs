#[cfg(feature = "sanrs")]
use {
    crate::sanity::client::SanityClient,
    crate::sanrs::{cli, config, query_shell},
    clap::Parser,
    std::env,
    std::fs,
    std::fs::File,
    std::io::prelude::*,
    std::path::Path,
    std::process,
    std::process::Command,
};

// TODO: Need to have an automatic way of joining the "sanity/.sanityrc" so that it is not
// hardcoded everywhere

/// Checks whether the config file exists
fn config_exists() -> bool {
    let home = env::var("HOME").expect("Home is not properly defined");
    let file_path = Path::new(&home).join("sanity/.sanityrc");

    let res = std::fs::metadata(file_path);

    match res {
        Ok(res) => true,
        Err(res) => false,
    }
}

/// Returns a SanityClient from the config file
fn get_client(profile:String) -> Result<SanityClient, Box<dyn std::error::Error>> {
    let config = config::find_profile(profile.clone());
    if let Some(config) = config {
        let token = config.env.api_token;
        let dataset = config.env.dataset;
        let project = config.env.project_id;

        let client = SanityClient::new(token, dataset, project);

        Ok(client)
    } else {
        eprintln!("Error: no profile with name {}",profile.clone());
        Err(format!("Errorr could not find the profile name {}", profile).into())
    }

}

// TODO: fill out docs
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Config(cmds) => match cmds {
            cli::ConfigCmds::Init(args) => {
                let name = args.name;
                let token = args.token;
                let project = args.project;
                let dataset = args.dataset;
                let toml = config::create_env_toml(name, token, dataset, project);

                // println!("Name: {:?}, token: {:?}, project: {:?}, dataset: {:?}, toml: {:?}", name, token, project, dataset, toml);
                // println!("toml: {:?}", toml);

                config::create_file(&toml)?;

                println!("Successfully created .sanityrc")
            }

            cli::ConfigCmds::Add(args) => {
                if !config_exists() {
                    eprintln!("The .sanityrc file does not exist. To initialize the config file, run `sanrs config init` to establish a profile");
                    process::exit(1)
                }
                let name = args.name;
                let token = args.token;
                let project = args.project;
                let dataset = args.dataset;
                let toml = config::create_env_toml(name, token, dataset, project);

                config::append_config(&toml)?;
                println!("Added new group to sanity config")

            }
            cli::ConfigCmds::View => {
                let home = env::var("HOME").expect("Home is not properly defined");
                let file_path = Path::new(&home).join("sanity/.sanityrc");
                let output = Command::new("cat")
                    .args([file_path.into_os_string().into_string().unwrap()])
                    .output()
                    .expect("Failed to view config file");

                
                let output_str = String::from_utf8_lossy(&output.stdout);
                println!("{}", output_str)
            }
            // TODO: Add the ability to launch different editors, not just vim
            cli::ConfigCmds::Edit => {
                let home = env::var("HOME").expect("Home is not properly defined");
                let file_path = Path::new(&home).join("sanity/.sanityrc");
                Command::new("vim")
                    .args([file_path.into_os_string().into_string().unwrap()])
                    .status()
                    .expect("Failed to edit config file");
                println!("Successfully edited config file")

            }
            cli::ConfigCmds::Remove(args) => {
                if !config_exists() {
                    eprintln!("The .sanityrc file does not exist. To initialize the config file, run `sanrs config init` to establish a profile");
                    process::exit(1)
                }
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
        cli::Commands::Query(args) => {
            if !config_exists() {
                eprintln!("The .sanityrc file does not exist. To initialize the config file, run `sanrs config init` to establish a profile");
                process::exit(1)
            }
            let interactive = args.interactive;
            let profile = args.profile;
            let client = get_client(profile)?;
            if interactive {
                println!("Entering interactive query shell");
                // spin up the query shell
                let _ = query_shell::run_shell(&client).await;
            } else {
                todo!();
            }
        }
        _ => {
            println!("No arguments entered");
        }
    }
    Ok(())
}
