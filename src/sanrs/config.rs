#[cfg(feature = "sanrs")]
use {
    crate::SanityClient,
    serde::{Deserialize, Serialize, Serializer},
    std::collections::{BTreeMap, HashMap},
    std::env,
    std::fs,
    std::fs::DirBuilder,
    std::fs::{File, OpenOptions},
    std::io::prelude::*,
    std::io::Write,
    std::path::Path,
    toml,
    toml::Value,
};

const DIR_NAME: &str = "sanity";
const FILE_NAME: &str = ".sanityrc";

#[derive(Serialize, Deserialize, Clone)]
pub struct Environment {
    pub api_token: String,
    pub dataset: String,
    pub project_id: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub env_name: String,
    pub env: Environment,
}

impl Serialize for Config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = BTreeMap::new();
        map.insert(&self.env_name, &self.env);
        map.serialize(serializer)
    }
}

/// Helper function that creates the TOML string that will be used to write to the config file
///
/// * `name`: The title of the toml section
/// * `token`: The api token to use for the section
/// * `dataset`: The dataset to use for the section
/// * `project`: The project ID to use for the section
pub fn create_env_toml(name: String, token: String, dataset: String, project: String) -> String {
    let sanity_env = Environment {
        api_token: token,
        dataset: dataset,
        project_id: project,
    };
    let config = Config {
        env_name: name,
        env: sanity_env,
    };

    let res = toml::to_string(&config);
    res.unwrap()
}

/// Creates the config file and writes the provided TOML string to it
///
/// * `toml_string`: The TOML string to write to the .sanityrc config file
pub fn create_file(toml_string: &String) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let dir_path = Path::new(&home_dir).join(DIR_NAME);
    // let file_path = Path::new(&home_dir).join(".sanity/.sanityrc");
    let file_path = Path::new(&home_dir).join(DIR_NAME).join(FILE_NAME);
    // create the directory first
    println!("The file path is {:?}", file_path);

    DirBuilder::new().recursive(true).create(&dir_path)?;

    let mut file = File::options().create(true).write(true).open(file_path)?;

    file.write_all(toml_string.as_bytes())?;
    file.flush()?;
    Ok(())
}

/// Adds a new group to the .sanityrc file
///
/// * `toml_string`: The toml of the new group to create
pub fn append_config(toml_string: &String) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let dir_path = Path::new(&home_dir).join(DIR_NAME);
    // let file_path = Path::new(&home_dir).join(".sanity/.sanityrc");
    let file_path = Path::new(&home_dir).join(DIR_NAME).join(FILE_NAME);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;

    file.write("\n".as_bytes())?;
    file.write(toml_string.as_bytes())?;
    file.flush()?;
    Ok(())
}

/// Reads the content of the .sanityrc file and returns the toml in the Config format
// pub fn read_config() -> Result<Vec<Config>, Box<dyn std::error::Error>> {
//     let home_dir = env::var("HOME")?;
//     let file_path = Path::new(&home_dir).join(DIR_NAME).join(FILE_NAME);
//     let file_contents = std::fs::read_to_string(file_path)?;
//     let toml_data:Config = toml::from_str(&file_contents)?;
//
//     return Ok(toml_data)
// }

/// Fetches the
//NOTE: this has been adjusted to return a vector of configs not just a single one
pub fn read_config() -> Result<Vec<Config>, Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let file_path = Path::new(&home_dir).join(DIR_NAME).join(FILE_NAME);
    let file_contents = std::fs::read_to_string(file_path)?;
    let raw_configs: HashMap<String, Environment> = toml::from_str(&file_contents)?;
    let configs: Vec<Config> = raw_configs
        .into_iter()
        .map(|(env_name, env)| Config { env_name, env })
        .collect();
    Ok(configs)
}

pub fn find_profile(group_name: String) -> Option<Config> {
    let mut groups = read_config().ok()?;
    let mut filtered = groups
        .clone()
        .into_iter()
        .filter(|config| config.env_name == group_name)
        .collect::<Vec<_>>();

    if filtered.clone().len() == 1 {
        Some(filtered.swap_remove(0))
    } else if filtered.len() > 1 {
        eprintln!("There are more than 1 groups with the name {}", group_name);
        None
    } else {
        None
    }
}
