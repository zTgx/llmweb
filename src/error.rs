use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlmWebError {
    #[error("Browser error: {0}")]
    Browser(String),

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Model client error: {0}")]
    ModelClient(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, LlmWebError>;
