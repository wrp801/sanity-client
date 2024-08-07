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
    let patch = json!({"description": "Patched description",
                        "name": "Patched Name",
                    });
    // let query = "*[_type == 'blueprints' && name == 'Test Blueprint']";

    let id = "5yEnY5RWDfL2hzwFTFNL3W".to_string();

    let delete_response = client
        .mutate()
        .delete() 
        .id(&id)
        .execute()
        .await;

    println!("{:?} ", delete_response);
}


