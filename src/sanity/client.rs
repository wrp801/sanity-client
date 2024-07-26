extern crate reqwest;
extern crate serde_json;

// use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value, Map, json};
use serde::Deserialize;
use reqwest::Client;
// use log::{info, debug, error, warn};

// use crate::sanity::errs::SanityError;
use crate::sanity::endpoints::{MutateEndpoint, QueryEndpoint};


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

#[derive(Debug)]
pub struct SanityClient<'a> {
    token: &'a str,
    dataset: &'a str, 
    project: &'a str,
}

impl <'a> SanityClient<'a> {
    pub fn new(token: &'a str, dataset: &'a str, project: &'a str) -> SanityClient<'a> {
        let client = Client::new();
        SanityClient {
            token: token,
            dataset: dataset,
            project: project
        }
    }

    pub fn mutate(&self) -> MutateEndpoint {
        MutateEndpoint::new(self.token, self.dataset, self.project)
    }

    pub fn query(&self) -> QueryEndpoint {
        QueryEndpoint::new(&self.token, &self.dataset, &self.project)
    }
}
