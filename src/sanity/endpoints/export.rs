extern crate reqwest;
extern crate serde_json;

use serde_json::Value;
use log::info;
use std::path::PathBuf;
use std::io::BufWriter;


use crate::sanity::endpoints::endpoint::Endpoint;
use crate::sanity::errs::SanityError;

pub struct ExportBuilder<'a> {
    endpoint: &'a ExportEndpoint<'a>,
    doc_type: Option<String>,
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

    pub async fn fetch(&mut self) -> Result<(), SanityError> {
        let mut form = reqwest::multipart::Form::new();
        let form_text = self.doc_type.clone().unwrap();
        form = form.text("types", form_text);

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
                let body = response.text().await?;
                let json: Value = serde_json::from_str(&body)?;

                if let Some(_filename) = self.filename {
                    self.data = Some(json.clone());
                } else {
                    println!("Data from Sanity:");
                    println!("{}", json)
                }

                Ok(())
                } else {
                    eprintln!("Export request failed with status code: {}", response.status());
                    return Err(SanityError::ExportError(String::from("Export request failed")));
                }
            },
            Err(e) => {
                eprintln!("Export request failed: {:?}", e);
                return Err(SanityError::ExportError(format!("Export request failed: {:?}", e)));
            }
        }

    }

    pub fn write(&self) -> Result<(), SanityError> {
        if let Some(data) = &self.data {
            if let Some(filename) = self.filename {
                let file = std::fs::File::create(filename);
                let mut writer = BufWriter::new(file.unwrap());
                serde_json::to_writer(&mut writer, &self.data);
                Ok(())
            } else {
                Err(SanityError::ExportError(String::from("No filename provided for writing")))
            }
        } else {
            Err(SanityError::ExportError(String::from("No data to write")))
        }
    }
}

pub struct ExportEndpoint<'a> {
    token: &'a String,
    dataset: &'a String,
    project: &'a String,
    client: reqwest::Client,
    url: Option<String>,
    headers: Option<reqwest::header::HeaderMap>,
}

impl <'a> ExportEndpoint<'a> {
    pub fn new(token: &'a String, dataset: &'a String, project: &'a String) -> ExportBuilder<'a> {
        let export_endpoint = ExportEndpoint {
            token: token,
            dataset: dataset,
            project: project,
            client: reqwest::Client::new(),
            url: Some(Endpoint::Export.get_url(project, dataset)),
            headers: {
                let mut headers = reqwest::header::HeaderMap::new();
                let header_value = format!("Bearer {}", token);
                headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&header_value).unwrap());
                headers.insert("Content-Type", reqwest::header::HeaderValue::from_static("application/json"));
                Some(headers)
            }
        };

        
        ExportBuilder::new(&export_endpoint)

    }
}


