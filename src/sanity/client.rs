extern crate reqwest;
extern crate serde_json;

use serde_json::{Value, Map, json};
use serde::Deserialize;
use reqwest::Client;

use crate::sanity::endpoints::query::QueryEndpoint;
use crate::sanity::endpoints::mutate::MutateEndpoint;
use crate::sanity::endpoints::export::{ExportEndpoint, ExportBuilder};
use crate::sanity::endpoints::projects::ProjectEndpoint;



#[derive(Deserialize, Debug)]
/// This is the struct that holds the result of a GROQ query to the Sanity API. 
/// 
/// * `ms`: The nubmer of milliseconds it took to get the result.
/// * `query`: The GROQ query that was executed
/// * `result`: The result of the query, containing the JSON data
pub struct QueryResult {
    pub ms: usize,
    pub query: String, 
    pub result: Vec<Value>
}

#[derive(Debug)]
/// This is the main struct that holds the Sanity client.
///
/// * `token`: The token to authenticate with the Sanity API
/// * `dataset`: The dataset to use
/// * `project`: The project ID to use
///
/// This struct is used to create a new client, and then use the client to create queries and mutations.
/// USAGE:
pub struct SanityClient {
    token: String,
    dataset: String, 
    project: String,
}

impl SanityClient {
    pub fn new(token: String, dataset: String, project: String) -> SanityClient {
        let client = Client::new();
        SanityClient {
            token: token,
            dataset: dataset,
            project: project
        }
    }

    pub fn mutate(&self) -> MutateEndpoint {
        MutateEndpoint::new(&self.token, &self.dataset, &self.project)
    }

    pub fn query(&self) -> QueryEndpoint {
        QueryEndpoint::new(&self.token, &self.dataset, &self.project)
    }

    pub fn export(&self) -> ExportBuilder  {
        let token = self.token.clone();
        let dataset = self.dataset.clone();
        let project = self.project.clone();
        ExportEndpoint::new(token, dataset, project)
    }

    pub fn projects(&self) -> ProjectEndpoint {
        let token = self.token.clone();
        let dataset = self.dataset.clone();
        let project = self.project.clone();
        ProjectEndpoint::new(token ,dataset, project)
    }
}
