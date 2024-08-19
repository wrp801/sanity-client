extern crate reqwest;
extern crate serde_json;

use serde_json::{Value, json};


use crate::sanity::endpoints::endpoint::{Endpoint, QueryResult};
use crate::sanity::errs::SanityError;


pub struct ExportEndpoint<'a> {
    token: &'a String,
    dataset: &'a String,
    project: &'a String,
    client: reqwest::Client,
    url: Option<String>,
    headers: Option<reqwest::header::HeaderMap>,
}

impl <'a> ExportEndpoint<'a> {
    pub fn new(token: &'a String, dataset: &'a String, project: &'a String) -> Self {
        ExportEndpoint {
            token: token,
            dataset: dataset,
            project: project,
            client: reqwest::Client::new(),
            url: Some(Endpoint::Export.get_url(project, dataset)),
            headers: {
                let mut headers = reqwest::header::HeaderMap::new();
                let header_value = format!("Bearer {}", token);
                headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&header_value).unwrap());
                headers.insert("Content-Type", reqwest::header::HeaderValue::from_static("application/json"));
                Some(headers)
            }
        }
    }

    pub async fn fetch(&self) -> Result<Value, SanityError> {
        let url = self.url.clone().expect("Query URL is not proplery set");
        let headers = self.headers.clone().expect("Headers are not properly set");
        let results = self 
            .client
            .get(url)
            .headers(headers)
            .send()
            .await;

        match results {
            Ok(response) => {
                if !response.status().is_success() {
                    return Err(SanityError::ExportError(format!("Export failed with status code: {}", response.status())));
                }
                let json = response.json::<Value>().await;
                match json {
                    Ok(data) => Ok(data),
                    Err(e) => Err(SanityError::ParseError(e.to_string()))
                }
            },
            Err(e) => {
                eprintln!("Error in exporting dataset: {:?}", e);
                Err(SanityError::ExportError(e.to_string()))
            }
        }
    }
}


