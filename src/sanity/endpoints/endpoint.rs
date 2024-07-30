use serde::Deserialize;
use serde_json::Value;

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
    pub fn get_url(&self, project: &str, dataset: &str) -> String {
        match self {
            Endpoint::Actions => format!("https://{}.api.sanity.io/v2021-10-21/data/actions/{}", project, dataset),
            Endpoint::Export => format!("https://{}.api.sanity.io/v2021-10-21/data/export/{}", project, dataset),
            Endpoint::Mutate => format!("https://{}.api.sanity.io/v2021-10-21/data/mutate/{}", project, dataset),
            Endpoint::Projects => format!("https://api.sanity.io/v2021-10-21/projects"),
            Endpoint::Query => {
                let now = chrono::Utc::now();
                let date_str = now.format("%Y-%m-%d").to_string();
                format!("https://{}.api.sanity.io/v{}/data/query/{}", project, date_str, dataset)

            }
        }
    }
}

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

