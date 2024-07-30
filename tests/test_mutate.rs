use sanity_client::sanity::client::SanityClient;
use dotenv::dotenv;
use std::env;

fn setup<'a>() -> SanityClient<'a> {
    dotenv().ok();
    let token = env::var("SANITY_TOKEN").unwrap();
    let dataset = env::var("SANITY_DATASET").unwrap();
    let project = env::var("SANITY_PROJECT").unwrap();
    SanityClient::new(&token, &dataset, &project)
}


#[tokio::test]
async fn test_successful_create() {
    let client = setup();

}
