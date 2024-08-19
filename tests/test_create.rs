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
async fn test_successful_create() {
    let client = setup();

    let doc = json!({
        "_type": "blueprints",
        "name": "TEST ME",
        "description": "This is a test document"
    });

    let res = client
        .mutate()
        .create(doc)
        .await;

    assert!(res.is_ok());

    // delete the document 
    let query = "*[_type == 'blueprints' && name == 'TEST ME']".to_string();

    let delete_res = client 
        .mutate()
        .delete()
        .query(&query)
        .execute()
        .await;

    assert!(delete_res.is_ok());

}

