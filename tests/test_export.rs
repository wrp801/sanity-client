use sanity_client::sanity::client::SanityClient;
use dotenv::dotenv;
use std::env;
use serde_json::json;

fn setup() -> SanityClient {
    dotenv().ok();
    let token = env::var("SANITY_TOKEN").unwrap();
    let dataset = env::var("SANITY_DATASET").unwrap();
    let project = env::var("SANITY_PROJECT").unwrap();
    SanityClient::new(token, dataset, project)
}


#[tokio::test] 
async fn test_successful_export_no_write() {
    let client = setup();

    let doc_types = vec!("blueprints");
    let res = client
        .export() 
        .doc_type(doc_types)
        .fetch();


}



