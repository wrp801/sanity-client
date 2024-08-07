extern crate reqwest;
extern crate serde_json;

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value, Map, json};
use reqwest::Client;
use log::{info, debug, error, warn};

use crate::sanity::endpoints::endpoint::Endpoint;
use crate::sanity::errs::SanityError;
use crate::sanity::endpoints::actions::delete::DeleteBuilder;
use crate::sanity::endpoints::actions::patch::PatchBuilder;


pub struct MutateEndpoint<'a> {
    pub token: &'a String,
    pub dataset: &'a String, 
    pub project: &'a String,
    pub client: Client,
    pub url: Option<String>,
    pub headers: Option<HeaderMap>,
}

impl<'a> MutateEndpoint<'a> {
    pub fn new(token: &'a String, dataset: &'a String, project: &'a String) -> Self {
        MutateEndpoint {
            token: token,
            dataset: dataset,
            project: project,
            client: Client::new(),
            url: Some(Endpoint::Mutate.get_url(project, dataset)),
            headers: {
                let mut headers = HeaderMap::new();
                let header_value = format!("Bearer {}", token);
                headers.insert("Authorization", HeaderValue::from_str(&header_value).unwrap() );
                headers.insert("Content-Type", HeaderValue::from_static("application/json"));
                Some(headers)
            }
        }
    }

    pub async fn create(&self, doc: Value) -> Result<Value, SanityError> {
        let payload = json!({
            "mutations": [
                {
                    "create": doc
                }
            ]
        });
        let url = self.url.as_ref().expect("Mutate URL is not proplery set");
        let headers = self.headers.clone().expect("Headers are not properly set");
        let res = self.client.post(url)
            .headers(headers)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(response) => {
                let json = response.json::<Value>().await;
                match json {
                    Ok(json) => {
                        Ok(json)
                        
                    },
                    Err(e) => {
                        eprintln!("Error parsing response: {:?}", e);
                        Err(SanityError::ParseError(e.to_string()))
                    }
                }
            },
            Err(e) => {
                error!("Error creating document: {:?}", e);
                Err(SanityError::MutateError(e.to_string()))
            }
        }

    }

    pub async fn create_or_replace(&self, doc: Value) -> Result<Value, reqwest::Error> {
        let payload = json!({
            "mutations": [
                {
                    "createOrReplace": doc
                }
            ]
        });
        let url = self.url.as_ref().expect("Mutate URL is not proplery set");
        let headers = self.headers.clone().expect("Headers are not properly set");
        let res = self.client.post(url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    pub async fn create_if_not_exists(&self, doc: Value) -> Result<Value, reqwest::Error> {
        let payload = json!({
            "mutations": [
                {
                    "createIfNotExists": doc
                }
            ]
        });
        let url = self.url.as_ref().expect("Mutate URL is not proplery set");
        let headers = self.headers.clone().expect("Headers are not properly set");
        let res = self.client.post(url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    pub fn patch(&'a self) -> PatchBuilder<'a> {
        PatchBuilder::new(self)
    }
    pub fn delete(&self) -> DeleteBuilder {
        DeleteBuilder::new(self)
    }
}

