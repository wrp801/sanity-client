extern crate reqwest;
extern crate serde_json;

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value, Map, json};
use reqwest::Client;
use log::{info, debug, error, warn};

use crate::sanity::endpoints::endpoint::Endpoint;
use crate::sanity::errs::SanityError;


pub struct MutateEndpoint<'a> {
   token: &'a String,
    dataset: &'a String, 
    project: &'a String,
    client: Client,
    url: Option<String>,
    headers: Option<HeaderMap>,

}

pub struct PatchBuilder<'a> {
    endpoint:  &'a MutateEndpoint<'a>,
    id: &'a str,
    set: Option<Value>,
    unset: Option<Value>,
    payload: Option<Value>

}


impl <'a> PatchBuilder<'a>  {
   pub fn new(endpoint: &'a MutateEndpoint,id: &'a str) -> Self {
         PatchBuilder {

            endpoint: endpoint,
            id: id,
            set: None,
            unset: None,
            payload: None
        }
    }

    pub fn set(&mut self, set: Value) -> &mut Self {
        self.set = Some(set);
        self
    }

    pub fn unset(&mut self, unset: Value) -> &mut Self {
        self.unset = Some(unset);
        self
    }

    pub fn build(&mut self) -> &mut Self {
        let mut patch = Map::new();
        patch.insert("id".to_string(), Value::String(self.id.to_string()));
        if let Some(set) = &self.set {
            patch.insert("set".to_string(), set.clone());
        }
        if let Some(unset) = &self.unset {
            patch.insert("unset".to_string(), unset.clone());
        }
        self.payload = Some(Value::Object(patch));
        self
    }

    async fn apply(&self) -> Result<Value, reqwest::Error> {
        let temp_payload = self.payload.clone();
        let payload = json!({
            "mutations": [
                {
                    "patch": temp_payload.unwrap()
                }
            ]
        });
        let url = self.endpoint.url.as_ref().expect("Mutate URL is not proplery set");
        let headers = self.endpoint.headers.clone().expect("Headers are not properly set");
        let res = self.endpoint.client.post(url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;
        let json: Value = res.json().await?;
        Ok(json)
    }
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

    pub fn patch(&'a self, id:&'a str) -> PatchBuilder<'a> {
        PatchBuilder::new(self, id)
    }
}

