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
async fn test_sanity_fetch() {
    // let token = env::var("SANITY_TOKEN").unwrap();
    // let dataset = env::var("SANITY_DATASET").unwrap();
    // let project = env::var("SANITY_PROJECT").unwrap();
    let token = "skmeoSwqPMxAk4knoltd4QYdOEBt1kcbpVAuNK1rjHMFASEr98v09LwpgcP4SUzIbiilHOUW2nGauqU4mZlL75hFFIXLLFkP3Or5VNOj7oZBGeoWifPRW2JWCI0S076yRFaiydChbod79f0U5juBciZkdYGw03ZWu10MmaAfLOf0aubASPzo";
    let dataset = "dev";
    let project = "2xyydva6";

    let client = SanityClient::new(&token, &dataset, &project);
    // let client = setup();
    let query = "*[_type == 'blueprints' && name match('Excel')]";
    let result = client.query().fetch(query).await.unwrap();
}


// async fn test_create() {
//     let client = setup();
//     assert!(result.result.len() > 0);
// }
//
//





