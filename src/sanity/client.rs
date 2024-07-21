extern crate reqwest;
extern crate serde_json;

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value, Map, json};
use serde::Deserialize;
use reqwest::Client;
use log::{info, debug, error, warn};

use crate::sanity::errs::SanityError;



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
/// 
///
/// * `token`: 
/// * `dataset`: 
/// * `project`: 
/// * `query_url`: 
/// * `mutate_url`: 
/// * `client`: 
/// * `doc_id`: 
pub struct SanityClient {
    token: String,
    dataset: String, 
    project:String,
    query_url: String,
    mutate_url: String,
    client: Client,
    doc_id: Option<String>,
    body: Option<Value>,


}

impl SanityClient {
    pub fn new(token: String, dataset: String, project: String) -> Self {
        let headers = {
            let mut headers = HeaderMap::new();
            headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
            headers.insert("Content-Type", HeaderValue::from_static("application/json"));
            headers
        };
        let query_url = format!( "https://{}.api.sanity.io/v2021-10-21/data/query/{}", project, dataset);
        let mutate_url = format!( "https://{}.api.sanity.io/v2021-10-21/data/mutate/{}", project, dataset);
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let doc_id = None;
        let body = None;

        Self {
            token,
            dataset,
            project,
            query_url,
            mutate_url,
            client,
            doc_id,
            body
        }
    }

    /// Executes a GROQ query against the Sanity API and returns the result.
    ///
    /// * `query`: The GROQ query to execute
    pub async fn fetch(&self, query:&str) -> Result<QueryResult, SanityError> {
        let mut payload = Map::new();
        payload.insert("query".to_string(), Value::String(query.to_string()));

        let response = self.client 
            .post(&self.query_url)
            .json(&payload)
            .send()
            .await;

        let response = match response {
            Ok(response) => response,
            Err(e) => {
                error!("Error fetching data from Sanity: {:?}", e);
                return Err(SanityError::QueryError(e.to_string()));
            }
        };

        let results = match response.json::<QueryResult>().await {
            Ok(results) => results,
            Err(e) => {
                error!("Error parsing JSON response from Sanity: {:?}", e);
                return Err(SanityError::ParseError(e.to_string()));
            }
        };
        Ok(results)
    }

    pub fn patch(mut self, doc_id: &str) -> Self {
        self.doc_id = Some(doc_id.to_string());
        self
    }

    /// Helper method for setting the body of a document to be created
    ///
    /// * `body`: The JSON value to set as the body of the document
    pub fn body(mut self, body: &Value) -> Self {
        self.body = Some(body.clone());
        self
    }

    // NOTE: methods for patches should follow this: https://www.sanity.io/docs/http-patches
    

    ///  Sets a field in a document to a new value.
    ///
    /// * `field`: Name of the field to set
    /// * `value`: The new value to set the field to
    pub async fn set(&self, field:&str, value:&str) -> Result<QueryResult, SanityError> {
        // check that the doc ID is set before proceeding 
        match &self.doc_id {
            Some(doc_id) => {
                let id = self.doc_id.as_ref().unwrap();
                let set_json = json!({
                    "mutations": [
                        {
                            "patch": {
                                "id": id,
                                "set": {
                                    field: value
                                }
                            }
                        }
                    ]
                });
                let results = self.client 
                    .post(&self.mutate_url)
                    .body(serde_json::to_string(&set_json).unwrap())
                    .send()
                    .await?
                    .json::<QueryResult>()
                    .await?;
                Ok(results)
        
            },
            None => {
                Err(SanityError::NoDocIdError("No document ID set for patching".to_string()))
            }
        }
    }

    
    /// Creates a new document in the dataset with the provided body from the preceeding `body`
    /// method.
    pub async fn create(&self) -> Result<Value, SanityError> {
        let body_clone = self.body.clone();
        match body_clone {
            Some(body) => {
                let create_json = json!({
                    "mutations": [
                        {
                            "create": body
                        }
                    ]
                });
                let response = self.client
                    .post(&self.mutate_url)
                    .body(serde_json::to_string(&create_json).unwrap())
                    .send()
                    .await;

                let response = match response {
                    Ok(response) => response, 
                    Err(e) => {
                        error!("Error creating document in Sanity: {:?}", e);
                        return Err(SanityError::MutateError(e.to_string()));
                    }
                };

                let results = match response.json::<Value>().await {
                    Ok(results) => results,
                    Err(e) => {
                        error!("Error parsing JSON response from Sanity: {:?}", e);
                        return Err(SanityError::ParseError(e.to_string()));
                    }
                };
                Ok(results)
            },
            None => {
                Err(SanityError::MutateError("No body provided for create. Use the body() method with a valid JSON value".to_string()))
            }
        }
    }

    // pub async fn delete(&self, doc_id:&str) -> Result<QueryResult, SanityError> {
    //     let delete_json = json!({
    //         "mutations": [
    //             {
    //                 "delete": {
    //                     "id": doc_id
    //                 }
    //             }
    //         ]
    //     });
    //
    // }
}



