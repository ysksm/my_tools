use thiserror::Error;

#[derive(Error, Debug)]
pub enum JiraSyncError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Database error: {0}")]
    Database(#[from] duckdb::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("JIRA API error: {0}")]
    JiraApi(String),
}

pub type Result<T> = std::result::Result<T, JiraSyncError>;
