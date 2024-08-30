use sanity_client::sanity::client::SanityClient;
use dotenv::dotenv;
use std::env;
use serde_json::json;
use std::path::PathBuf;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("SANITY_TOKEN").unwrap();
    let dataset = std::env::var("SANITY_DATASET").unwrap();
    let project = std::env::var("SANITY_PROJECT").unwrap();
    let client = SanityClient::new(token, dataset, project);

    let res = client 
        .projects()
        .list()
        .await
        .unwrap();

    let pretty = serde_json::to_string_pretty(&res).unwrap();
    println!("{}", pretty);
}


