extern crate reqwest;
extern crate serde_json;

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value, Map, json};
use serde::Deserialize;
use reqwest::Client;
use log::{info, debug, error, warn};
use chrono::Datelike;


#[derive(Deserialize, Debug)]
/// This is the struct that holds the result of a query to the Sanity API. 
/// 
/// * `ms`: The nubmer of milliseconds it took to get the result.
/// * `query`: The GROQ query that was executed
/// * `result`: The result of the query, containing the JSON data
pub struct QueryResult {
    pub ms: usize,
    pub query: String, 
    pub result: Vec<Value>
}


pub enum Endpoint {
    Actions,
    Export,
    Mutate,
    Projects,
    Query,
}

impl Endpoint {
    /// Returns the appropriate URL for the endpoint
    ///
    /// * `project`: The project ID
    /// * `dataset`: The name of the dataset
    fn get_url(&self, project: &str, dataset: &str) -> String {
        match self {
            Endpoint::Actions => format!("https://{}.api.sanity.io/v2021-10-21/data/actions/{}", project, dataset),
            Endpoint::Export => format!("https://{}.api.sanity.io/v2021-10-21/data/export/{}", project, dataset),
            Endpoint::Mutate => format!("https://{}.api.sanity.io/v2021-10-21/data/mutate/{}", project, dataset),
            Endpoint::Projects => format!("https://api.sanity.io/v2021-10-21/projects"),
            Endpoint::Query => {
                let now = chrono::Utc::now();
                let date_str = now.format("%Y-%m-%d").to_string();
                format!("https://{}.api.sanity.io/{}/data/query/{}", project, date_str, dataset)

            }
        }
    }
}


pub struct MutateEndpoint<'a> {
    token: &'a str,
    dataset: &'a str, 
    project:&'a str,
    client: Client,
    url: Option<String>,
    headers: Option<HeaderMap>,

}

struct PatchBuilder<'a> {
    endpoint: &'a MutateEndpoint<'a>,
    id: &'a str,
    set: Option<Value>,
    unset: Option<Value>,
    payload: Option<Value>

}


impl <'a> PatchBuilder<'a>  {
    fn new(endpoint: &'a MutateEndpoint,id: &'a str) -> Self {
        PatchBuilder {

            endpoint: endpoint,
            id: id,
            set: None,
            unset: None,
            payload: None
        }
    }

    fn set(&mut self, set: Value) -> &mut Self {
        self.set = Some(set);
        self
    }

    fn unset(&mut self, unset: Value) -> &mut Self {
        self.unset = Some(unset);
        self
    }

    fn build(&mut self) -> &mut Self {
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
    pub fn new(token: &'a str, dataset: &'a str, project: &'a str) -> Self {
        let client = Client::new();
        MutateEndpoint {
            token: token,
            dataset: dataset,
            project: project,
            client: Client::new(),
            url: Some(Endpoint::Mutate.get_url(project, dataset)),
            headers: {
                let mut headers = HeaderMap::new();
                headers.insert("Authorization", HeaderValue::from_str(&token).unwrap());
                headers.insert("Content-Type", HeaderValue::from_static("application/json"));
                Some(headers)
            }
        }
    }

    pub async fn create(&self, doc: Value) -> Result<Value, reqwest::Error> {
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
            .await?;
        let json: Value = res.json().await?;
        Ok(json)
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

    pub fn patch(&self, id:&'a str) -> PatchBuilder {
        PatchBuilder::new(self, id)
    }

}

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
                headers.insert("Authorization", HeaderValue::from_str(&token).unwrap());
                headers.insert("Content-Type", HeaderValue::from_static("application/json"));
                Some(headers)
            }
        }
    }

    pub async fn fetch(&self, query: &str) -> Result<QueryResult, reqwest::Error> {
        let payload = json!({
            "query": query
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
}
