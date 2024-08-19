extern crate reqwest;
extern crate serde_json;

use std::collections::HashMap;
use serde_json::{Value,  json};
use log::{info, error };
use std::path::PathBuf;

use crate::sanity::errs::SanityError;
use crate::sanity::endpoints::mutate::ExportEndpoint;

pub struct ExportBuilder<'a> {
    endpoint: &'a MutateEndpoint<'a>,
    doc_type: &'a String,
    data: Option<Value>,
    filename: Option<&'a PathBuf>
}

impl <'a> ExportBuilder<'a> {
    pub fn new(endpoint: &'a ExportEndpoint) -> Self{
        ExportBuilder{
            endpoint: endpoint,
            doc_type: None,
            data: None,
            filename: None
        }
    }

    pub fn doc_type(&mut self, doc_type: &'a Vec<String>) -> &mut Self {
        self.doc_type = Some(doc_type.join(","));
        self
    }

    pub fn data(&mut self, data: Value) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn filename(&mut self, filename: &'a PathBuf) -> &mut Self {
        self.filename = Some(filename);
        self
    }

    pub async fn fetch(&self) -> Result<(), SanityError> {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("types", self.doc_type);

        let url = self.endpoint.url.as_ref().expect("Mutate URL is not proplery set");
        let headers = self.endpoint.headers.clone().expect("Headers are not properly set");
        let client = reqwest::Client::new();
        let res = client.get(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Export request successful");
                let body = res.text().await?;
                let json: Value = serde_json::from_str(&body)?;

                if let Some(filename) = self.filename {
                    self.data = Some(json.clone());
                } else {
                    println!("Data from Sanity:");
                    println!("{}", json)
                }

                Ok(())
                } else {
                    error!("Export request failed with status code: {}", response.status());
                    return Err(SanityError::new("Export request failed"));
                }
            },
            Err(e) => {
                error!("Export request failed: {:?}", e);
                return Err(SanityError::new("Export request failed"));
            }
        }

    }

    pub fn write(&self) -> Result<(), SanityError> {
        if let Some(data) = &self.data {
            if let Some(filename) = self.filename {
                let file = std::fs::File::create(filename)?;
                serde_json::to_writer(file, data)?;
                Ok(())
            } else {
                Err(SanityError::new("No filename provided for writing"))
            }
        } else {
            Err(SanityError::new("No data to write"))
        }
    }
}
