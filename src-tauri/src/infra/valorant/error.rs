#[derive(thiserror::Error, Debug)]
pub enum APIError {
    #[error("API error: {0}")]
    APIError(String),
}
