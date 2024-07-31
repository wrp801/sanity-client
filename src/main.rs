use sanity_client::sanity::client::SanityClient;
use dotenv::dotenv;
use std::env;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("SANITY_TOKEN").unwrap();
    let dataset = std::env::var("SANITY_DATASET").unwrap();
    let project = std::env::var("SANITY_PROJECT").unwrap();
    let client = SanityClient::new(token, dataset, project);
    // let query = "*[_type == 'blueprints' && name match('Excel')]";
    let query = "*[_type == 'blueprints' && missing_the_closing_bracket";
    let result = client
        .query()
        .fetch(query)
        .await;


    match result {
        Ok(_) => {
            println!("Successfully executed query");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    // let create_json = json!({
    //     "_type": "blueprints",
    //     "name": "TESTME",
    //     "description": "A TEST FOR RUST",
    // });



}


