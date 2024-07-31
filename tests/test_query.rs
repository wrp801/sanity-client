use sanity_client::sanity::client::SanityClient;
use dotenv::dotenv;
use std::env;
use std::any::type_name;

fn setup() -> SanityClient {
    dotenv().ok();
    let token = env::var("SANITY_TOKEN").unwrap();
    let dataset = env::var("SANITY_DATASET").unwrap();
    let project = env::var("SANITY_PROJECT").unwrap();
    SanityClient::new(token, dataset, project)
}


#[tokio::test]
async fn test_sanity_fetch_successfull_query() {
    dotenv().ok();
    let client = setup();
    let query = "*[_type == 'blueprints' && name match('Excel')]";
    let result = client
        .query()
        .fetch(query)
        .await 
        .unwrap();

    assert!(result.result.len() > 0);
    
}

#[tokio::test]
async fn test_bad_query() {
    // This should return an error
    dotenv().ok();
    let client = setup();

    let query = "*[_type == 'blueprints' && missing_the_closing_bracket";

    let result = client.query().fetch(query).await;


    assert_eq!(result.is_ok(), false);

}


#[tokio::test] 
async fn test_good_query_no_results() {
    dotenv().ok();
    let client = setup();
    let query = "*[_type == 'blueprints' && name match('NO_MATCH')]";
    let result = client
        .query()
        .fetch(query)
        .await 
        .unwrap();

    assert_eq!(result.result.len(), 0);
}

