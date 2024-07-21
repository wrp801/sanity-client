use sanity_client::sanity::client::SanityClient;
use dotenv::dotenv;
use serde_json::json;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("SANITY_TOKEN").unwrap();
    let dataset = std::env::var("SANITY_DATASET").unwrap();
    let project = std::env::var("SANITY_PROJECT").unwrap();
    let client = SanityClient::new(token, dataset, project);
    let create_json = json!({
        "_type": "blueprints",
        "name": "TESTME",
        "description": "A TEST FOR RUST",
    });
    let result = client
        .body(&create_json)
        .create()
        .await
        .unwrap();

    println!("{:?}", result);
}


