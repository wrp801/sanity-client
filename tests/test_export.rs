use sanity_client::sanity::client::SanityClient;
use dotenv::dotenv;
use std::env;
use serde_json::json;
use std::path::PathBuf;
use std::fs;

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

    let doc_types = vec!(String::from("blueprints"));
    let path = PathBuf::from("tests/export_output.ndjson");

    let _ = client 
        .export()
        .doc_type(doc_types)
        .fetch()
        .await 
        .unwrap() 
        .write(path.clone());

    assert!(path.exists());

    let res = fs::remove_file(path);
    assert!(res.is_ok());
}




