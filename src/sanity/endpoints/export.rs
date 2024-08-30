
extern crate reqwest;
extern crate serde_json;

use serde_json::{Value, Deserializer};
use log::info;
use std::path::PathBuf;
use std::io::{BufWriter, Write};
use std::collections::HashMap;


use crate::sanity::endpoints::endpoint::Endpoint;
use crate::sanity::errs::SanityError;


pub struct ExportBuilder {
    endpoint: ExportEndpoint,
    doc_type: Option<String>,
    data: Option<Value>,
}


impl  ExportBuilder {
    pub fn new(endpoint: ExportEndpoint) -> Self{
        ExportBuilder{
            endpoint: endpoint,
            doc_type: None,
            data: None,
        }
    }

    /// The document types (as defined in the schema) to export
    ///
    /// * `doc_type`: The vector of document types to export
    pub fn doc_type(&mut self, doc_type: Vec<String>) -> &mut Self {
        self.doc_type = Some(doc_type.join(","));
        self
    }

    pub fn data(&mut self, data: Value) -> &mut Self {
        self.data = Some(data);
        self
    }

    /// Sends the request to the Sanity API to fetch the data. If the request is successful, the data is stored in the builder to be used later via the `write` or `print` methods
    pub async fn fetch(&mut self) -> Result<&mut Self, SanityError> {
        let url = self.endpoint.url.as_ref().expect("Export URL is not proplery set");
        let headers = self.endpoint.headers.clone().expect("Headers are not properly set");
        let mut params = HashMap::new();
        params.insert("types", self.doc_type.clone().unwrap());
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .query(&params)
            .headers(headers)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Export request successful");
                let body = response.text().await?;
                let json = Deserializer::from_str(&body).into_iter::<Value>();

                self.data = Some(json.collect::<Result<Value, _>>().unwrap());
                Ok(self)
                } else {
                    eprintln!("Export request failed with status code: {}", response.status());
                    return Err(SanityError::ExportError(response.text().await?));
                }
            },
            Err(e) => {
                eprintln!("Export request failed: {:?}", e);
                return Err(SanityError::ExportError(format!("Export request failed: {:?}", e)));
            }
        }

    }

    /// Writes the JSON data to a file
    ///
    /// * `filename`: The name of the file to write the JSON stream to
    pub fn write(&self, filename:PathBuf) -> Result<(), Box<dyn std::error::Error>>{
        let file = std::fs::File::create(filename);
        let mut writer = BufWriter::new(file.unwrap());
        for item in self.data.as_ref().unwrap().as_array().unwrap() {
            serde_json::to_writer(&mut writer, item)?;
            writer.write_all(b"\n")?;
        }
        
        // serde_json::to_writer(&mut writer, &self.data)?;
        Ok(())
    }

    /// Prints the JSON data from the API to the console in a pretty format
    pub fn print(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", serde_json::to_string_pretty(&self.data)?);
        Ok(())
    }
}

pub struct ExportEndpoint {
    token: String,
    dataset: String,
    project: String,
    client: reqwest::Client,
    url: Option<String>,
    headers: Option<reqwest::header::HeaderMap>,
}

impl ExportEndpoint {
    pub fn new(token: String, dataset: String, project: String) -> ExportBuilder {
        let export_endpoint = ExportEndpoint {
            token: token.clone(),
            dataset: dataset.clone(),
            project: project.clone(),
            client: reqwest::Client::new(),
            url: Some(Endpoint::Export.get_url(&project, &dataset)),
            headers: {
                let mut headers = reqwest::header::HeaderMap::new();
                let header_value = format!("Bearer {}", token);
                headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&header_value).unwrap());
                headers.insert("Content-Type", reqwest::header::HeaderValue::from_static("application/json"));
                Some(headers)
            },
        };

        ExportBuilder::new(export_endpoint)
    }
}



