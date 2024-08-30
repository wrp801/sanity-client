use serde_json::Value;

#[derive(Debug)]
pub enum SanityError {
    QueryError(String),
    MutateError(String),
    ClientError(String),
    ParseError(String),
    NoDocIdError(String),
    DeleteError(String),
    PatchError(String),
    ExportError(String),
    ProjectError(String),
}

impl From<reqwest::Error> for SanityError {
    fn from(error: reqwest::Error) -> Self {
        SanityError::ClientError(error.to_string())
    }

}

impl From<serde_json::Error> for SanityError {
    fn from(error: serde_json::Error) -> Self {
        SanityError::ParseError(error.to_string())
    }
}



