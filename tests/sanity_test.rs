use sanity_client::sanity::client::SanityClient;
use dotenv::dotenv;
use std::env;



fn setup() -> SanityClient {
    dotenv().ok();
    let token = env::var("SANITY_TOKEN").unwrap();
    let dataset = env::var("SANITY_DATASET").unwrap();
    let project = env::var("SANITY_PROJECT").unwrap();
    SanityClient::new(token, dataset, project)

}


#[tokio::test]
async fn test_sanity_fetch() {
    let client = setup();
    let query = "*[_type == 'blueprints' && name match('Excel')]";
    let result = client.fetch(query).await.unwrap();
    assert!(result.result.len() > 0);

}


async fn test_create() {
    let client = setup();
    assert!(result.result.len() > 0);
}







