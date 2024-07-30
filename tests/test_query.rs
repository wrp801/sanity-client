use sanity_client::sanity::client::SanityClient;
use dotenv::dotenv;
use std::env;




// fn setup<'a>() -> SanityClient<'a> {
//     dotenv().ok();
//     let token = env::var("SANITY_TOKEN").unwrap();
//     let dataset = env::var("SANITY_DATASET").unwrap();
//     let project = env::var("SANITY_PROJECT").unwrap();
//     SanityClient::new(&token, &dataset, &project)
//
// }
//

#[tokio::test]
async fn test_sanity_fetch_successfull_query() {
    dotenv().ok();
    let token = std::env::var("SANITY_TOKEN").unwrap();
    let dataset = std::env::var("SANITY_DATASET").unwrap();
    let project = std::env::var("SANITY_PROJECT").unwrap();
    let client = SanityClient::new(&token, &dataset, &project);
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
    let token = env::var("SANITY_TOKEN").unwrap();
    let dataset = env::var("SANITY_DATASET").unwrap();
    let project = env::var("SANITY_PROJECT").unwrap();

    let client = SanityClient::new(&token, &dataset, &project);

    let query = "*[_type == 'blueprints' && missing_the_closing_bracket";

    let result = client.query().fetch(query).await;


    assert_eq!(result.is_ok(), false);




}





// async fn test_create() {
//     let client = setup();
//     assert!(result.result.len() > 0);
// }
//
//





