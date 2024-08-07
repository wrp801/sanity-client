extern crate reqwest;
extern crate serde_json;

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value, Map, json};
use reqwest::Client;
use log::{info, debug, error, warn};

use crate::sanity::endpoints::endpoint::Endpoint;
use crate::sanity::errs::SanityError;
use crate::sanity::endpoints::mutate::MutateEndpoint;

pub struct DeleteBuilder<'a> {
    pub endpoint:  &'a MutateEndpoint<'a>,
    id: Option<&'a String>,
    query: Option<&'a String>,
    body: Option<Value>
}

impl <'a> DeleteBuilder<'a>  {
    pub fn new(endpoint: &'a MutateEndpoint) -> Self {
        DeleteBuilder {
            endpoint: endpoint,
            id: None,
            query: None,
            body: None
        }
    }

    pub fn id(&mut self, id: &'a String) -> &mut Self {
        self.id = Some(id);
        let payload = json!({
            "mutations": [
                {
                    "delete": {
                        "id": id
                    }
                }
            ]
        });
        self.body = Some(payload);
        self
    }

    pub fn query(&mut self, query: &'a String) -> &mut Self {
        self.query = Some(query);
        let payload = json!({
            "mutations": [
                {
                    "delete": {
                        "query": query
                    }
                }
            ]
        });
        self.body = Some(payload);
        self
    }

    pub async fn execute(&self) -> Result<Value, SanityError> {
        let url = self.endpoint.url.as_ref().expect("Mutate URL is not properly set");
        let headers = self.endpoint.headers.clone().expect("Headers are not properly set");

        let res = self.endpoint.client 
            .post(url)
            .headers(headers)
            .json(&self.body)
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
                eprintln!("Error deleting document: {:?}", e);
                Err(SanityError::DeleteError(e.to_string()))
            }
        }

    }
}
