use sanity_client::sanity::client::SanityClient;
use dotenv::dotenv;
use std::env;
use serde_json::json;
use std::path::PathBuf;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("SANITY_TOKEN").unwrap();
    let dataset = std::env::var("SANITY_DATASET").unwrap();
    let project = std::env::var("SANITY_PROJECT").unwrap();
    let client = SanityClient::new(token, dataset, project);

    let action = "write";
    let doc_types = vec!(String::from("freeBlueprints"));

    if action == "write" {
        let output = PathBuf::from("testoutput.ndjson");
        let _ = client 
            .export() 
            .doc_type(doc_types)
            .fetch()
            .await 
            .unwrap()
            .write(output);

        println!("Successfully wrote file to testoutput.ndjson");
    } else if action == "print" {
        let _ = client 
            .export() 
            .doc_type(doc_types)
            .fetch()
            .await
            .unwrap()
            .print();
    }

}


