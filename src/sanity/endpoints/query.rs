extern crate reqwest;
extern crate serde_json;

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value, Map, json};
use serde::Deserialize;
use reqwest::Client;
use log::{info, debug, error, warn};
use chrono::Datelike;

use crate::sanity::endpoints::endpoint::{Endpoint, QueryResult};
use crate::sanity::errs::SanityError;



pub struct QueryEndpoint {
    token: String,
    dataset: String, 
    project:String,
    client: Client,
    url: Option<String>,
    headers: Option<HeaderMap>,
}

impl QueryEndpoint {
    pub fn new(token: &str, dataset: &str, project: &str) -> Self {
        QueryEndpoint {
            token: token.to_string(),
            dataset: dataset.to_string(),
            project: project.to_string(),
            client: Client::new(),
            url: Some(Endpoint::Query.get_url(project, dataset)),
            headers: {
                let mut headers = HeaderMap::new();
                let header_value = format!("Bearer {}", token);
                headers.insert("Authorization", HeaderValue::from_str(&header_value).unwrap());
                headers.insert("Content-Type", HeaderValue::from_static("application/json"));
                Some(headers)
            }
        }
    }

    pub async fn fetch(&self, query: &str) -> Result<QueryResult, SanityError> {
        let payload = json!({
            "query": query
        });
        let url = self.url.clone().expect("Query URL is not proplery set");
        let headers = self.headers.clone().expect("Headers are not properly set");
        println!("URL: {:?}", url);
        println!("Headers: {:?}", headers);

        let results = self.client.post(url)
            .headers(headers)
            .json(&payload)
            .send()
            .await;


        match results {
            Ok(response) => {
                if !response.status().is_success() {
                    return Err(SanityError::QueryError(format!("Query failed with status: {:?} and reason: {:?}", response.status(), response.text().await.unwrap())));

                }
                let json_result = response.json::<QueryResult>().await;
                match json_result {
                    Ok(result) => {
                        Ok(result)
                    },
                    Err(e) => {
                        error!("Error parsing response: {:?}", e);
                        Err(SanityError::ParseError(e.to_string()))
                    }
                }

            },
            Err(e) => {
                error!("Error fetching data: {:?}", e);
                Err(SanityError::QueryError(e.to_string()))
            }
        }
    }
}





