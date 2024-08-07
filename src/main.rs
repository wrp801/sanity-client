use sanity_client::sanity::client::SanityClient;
use dotenv::dotenv;
use std::env;
use serde_json::json;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("SANITY_TOKEN").unwrap();
    let dataset = std::env::var("SANITY_DATASET").unwrap();
    let project = std::env::var("SANITY_PROJECT").unwrap();
    let client = SanityClient::new(token, dataset, project);
    // let query = "*[_type == 'blueprints' && name match('Excel')]";
    let patch = json!({"{description": "This is a test blueprint for integration tests. It has been patched."});
    let query = "*[_type == 'blueprints' && name == 'Test Blueprint']";

    let id = "5yEnY5RWDfL2hzwFTFNL3W".to_string();
    let patch_results = client 
        .mutate()
        .patch()
        // .query(&query.to_string())
        .id(&id)
        .set(patch)
        .build()
        .apply()
        .await;

    println!("patch results: {:?}", patch_results);

}


