extern crate reqwest;
extern crate serde_json;

use serde_json::{Value, Deserializer};
use log::info;

use crate::sanity::endpoints::endpoint::Endpoint;
use crate::sanity::errs::SanityError;



pub struct ProjectEndpoint {
    token: String,
    dataset: String,
    project: String,
    client: reqwest::Client,
    url: Option<String>,
    headers: Option<reqwest::header::HeaderMap>,

}

impl ProjectEndpoint {
    pub fn new(token: String ,dataset: String, project: String) -> Self {
        ProjectEndpoint {
            token: token.clone(),
            dataset: dataset.clone(),
            project: project.clone(),
            client: reqwest::Client::new(),
            url: Some(Endpoint::Projects.get_url(&project, &dataset)),
            headers: {
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
                headers.insert("Content-Type", reqwest::header::HeaderValue::from_str("application/json").unwrap());
                Some(headers)
            },
        }
    }
    /// Lists all the projects associated with the Sanity Studio
    pub async fn list(&self) -> Result<Value, SanityError> {
        let url = self.url.as_ref().expect("Project URL is not proplery set");
        let headers = self.headers.clone().expect("Headers are not properly set");

        let res = self.client
            .get(url)
            .headers(headers)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    info!("List request successful");
                    let json_response = response.json::<Value>().await;
                    match json_response {
                        Ok(data) => {
                            Ok(data)
                        },
                        Err(e) => {
                            Err(SanityError::ProjectError(format!("Error parsing response: {}", e)))
                        }
                    }

                } else {
                    Err(SanityError::ProjectError(format!("List request failed with status code: {}", response.status())))
                }
            },
            Err(response) => {
                Err(SanityError::ProjectError(format!("List request failed: {:?}", response)))
            }
        }
    }
    /// Retrieves a specific project and it's related fields and metadata by its ID
    ///
    /// * `project_id`: The ID of the project to fetch
    pub async fn retrieve(&self, project_id:String) -> Result<Value, SanityError> {
        let url = format!("{}/{}", self.url.as_ref().expect("Project URL is not proplery set"), project_id);
        let headers = self.headers.clone().expect("Headers are not properly set");

        let res = self.client
            .get(url)
            .headers(headers)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Retrieve request successful");
                    let json_response = response.json::<Value>().await;
                    match json_response {
                        Ok(data) => {
                            Ok(data)
                        },
                        Err(e) => {
                            Err(SanityError::ProjectError(format!("Error parsing response for retrieve endpoint: {}", e)))
                        }
                    }

                } else {
                    Err(SanityError::ProjectError(format!("Retrieve request failed with status code: {}", response.status())))
                }
            },
            Err(e) => {
                Err(SanityError::ProjectError(format!("Retrieve request failed: {:?}", e)))
            }
        }
    }

    pub async fn list_datasets(&self) {

    }
}
