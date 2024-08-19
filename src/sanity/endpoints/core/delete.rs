//! This module provides the delete functionality for the Sanity client.
//! The `DeleteBuilder` forms the delete request and allows for the deletion of sanity documents
//! by ID or query. 
//! **Usage**:
//! ``rust 
//! use sanity_client::sanity::client::SanityClient;
//! use dotenv::dotenv;
//!
//! async fn delete_by_id() {
//!    dotenv().ok();
//!    let token = std::env::var("SANITY_TOKEN").unwrap();
//!    let dataset = std::env::var("SANITY_DATASET").unwrap();
//!    let project = std::env::var("SANITY_PROJECT").unwrap();
//!    let client = SanityClient::new(token, dataset, project);
//!
//!    let id = '<document_id>'.to_string();
//!
//!    client
//!    .mutate() 
//!    .delete()
//!    .id(&id)
//!    .execute()
//!    .await
//! }
//!
//! async fn delete_by_query() {
//!   dotenv().ok();
//!    let token = std::env::var("SANITY_TOKEN").unwrap();
//!    let dataset = std::env::var("SANITY_DATASET").unwrap();
//!    let project = std::env::var("SANITY_PROJECT").unwrap();
//!    let client = SanityClient::new(token, dataset, project);
//!    let query = "*[_type == 'docs' && name == 'My Document']".to_string();
//!
//!    client 
//!    .mutate() 
//!    .delete()
//!    .query(&query)
//!    .execute()
//!    .await
//! }
//!
//!
//!
//!
//!
//!

extern crate reqwest;
extern crate serde_json;

use serde_json::{Value, json};

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

    /// This is the builder method for the document ID to delete.
    ///
    /// * `id`: The document ID to delete
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

    /// This is the builder method for a GROQ query to delete documents.
    ///
    /// * `query`: The valid GROQ query to delete docuemnts
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

    /// This is the method that sends the delete request to the Sanity API.
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
