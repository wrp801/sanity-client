extern crate reqwest;
extern crate serde_json;

use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value, Map, json};
use reqwest::Client;
use log::{info, debug, error, warn};

use crate::sanity::endpoints::endpoint::Endpoint;
use crate::sanity::errs::SanityError;
use crate::sanity::endpoints::mutate::MutateEndpoint;


pub struct PatchBuilder<'a> {
    endpoint:  &'a MutateEndpoint<'a>,
    id: Option<&'a String>,
    set: Option<Value>,
    unset: Option<&'a String>,
    payload: Option<Value>,
    query: Option<&'a String>,


}

impl <'a> PatchBuilder<'a>  {
   pub fn new(endpoint: &'a MutateEndpoint) -> Self {
         PatchBuilder {

            endpoint: endpoint,
            id: None,
            set: None,
            unset: None,
            payload: None,
            query: None,
        }
    }

    /// This builds the patch request for the SET action by providing the json value of the field
    /// name and the value to be set
    ///
    /// * `set`: The JSON value to be set
    pub fn set(&mut self, set: Value) -> &mut Self {
        let payload = json!({
            "set": set
        });
        self.set = Some(payload);
        self
    }

    /// This build the patch request for the UNSET action by providing the field name to be unset
    ///
    /// * `unset`: The name of the field to be unset. This will delete the existing value of the
    /// field
    pub fn unset(&mut self, unset: &'a String) -> &mut Self {
        self.unset = Some(unset);
        self
    }


    /// This builds the patch request by providing the query for the returning documents to be
    /// modified 
    ///
    /// * `query`: The GROQ query for the documents to be modified
    pub fn query(&mut self, query: &'a String) -> &mut Self {
        self.query = Some(query);
        self
    }
    /// This builds the patch request by providing the ID for the document to be
    /// modified.
    ///
    /// * `id`: The ID of the Sanity document to be modified
    pub fn id(&mut self, id: &'a String) -> &mut Self {
        self.id = Some(id);
        self
    }

    /// Builds the request for the patch based off of the query/id and set/unset
    pub fn build(&mut self) -> &mut Self {
        let mut patch = HashMap::new();
        if let Some(_id) = &self.id{
            patch.insert("id".to_string(), json!(self.id.unwrap()));
        }
        else if let Some(_query) = &self.query{
            patch.insert("query".to_string(), json!(self.query.unwrap()));
        }
        else {
            panic!("No ID or Query set for patch");
        }

        // check the set or unset values

        if let Some(_set) = &self.set {
            patch.insert("set".to_string(), json!(self.set.as_ref().unwrap()));
        }

        if let Some(_unset) = &self.unset {
            patch.insert("unset".to_string(), json!(self.unset.as_ref().unwrap()));
        }
        self.payload = Some(json!({
            "mutations": [
            {
                "patch": patch
            }
            ]
        }));
        self
    }


    /// Sends the current patch request through the client to the Sanity API
    pub async fn apply(&self) -> Result<Value, SanityError> {
        let url = self.endpoint.url.as_ref().expect("Mutate URL is not proplery set");
        let headers = self.endpoint.headers.clone().expect("Headers are not properly set");
        let res = self.endpoint.client.post(url)
            .headers(headers)
            .json(&self.payload)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Patch request successful");
                } else {
                    error!("Patch request failed: {:?}", response);
                    let resp_text = response.text().await;
                    return Err(SanityError::MutateError(resp_text.unwrap_or("".to_string())));
                }
                let json = response.json::<Value>().await;
                match json {
                    Ok(json) => Ok(json),
                    Err(e) => {
                        eprintln!("Error parsing response: {:?}", e);
                        Err(SanityError::ParseError(e.to_string()))
                    }
                }
            }
            Err(e) => {
                eprintln!("Error patching document: {:?}", e);
                Err(SanityError::MutateError(e.to_string()))
            }
        }
    }
}


