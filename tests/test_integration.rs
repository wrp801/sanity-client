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
async fn integration_test_create_document_check_results_patch_and_delete() {
    // This test will create a document, check that it was created, patch the document, check that
    // the patch was successful, and then delete the document

    dotenv().ok();
    let client = setup();
    // create the document
    let doc = 
        json!({"_type": "blueprints",
        "name": "Test Blueprint",
        "description": "This is a test blueprint for integration tests",
    });

    let result = client.mutate().create(doc)
        .await;

    assert!(result.is_ok());

    // check that the document was created
    let query = "*[_type == 'blueprints' && name == 'Test Blueprint']";
    let query_results = client
        .query()
        .fetch(&query)
        .await;

    assert!(query_results.is_ok());
    assert!(query_results.unwrap().result.len() > 0);

    // patch the document 
    let patch = json!({"set": {"description": "This is a test blueprint for integration tests. It has been patched."}});
    let patch_results = client 
        .mutate()
        .patch()
        .query(&query.to_string())
        .set(patch)
        .build()
        .apply()
        .await;

    assert!(patch_results.is_ok());

    // check that the patch was successful 

    let query_results = client
        .query()
        .fetch(&query)
        .await
        .unwrap()
        .result;

    assert_eq!(query_results[0]["description"], "This is a test blueprint for integration tests. It has been patched.");
}





