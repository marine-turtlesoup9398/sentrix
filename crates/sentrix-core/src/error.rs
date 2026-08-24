use thiserror::Error;

#[derive(Error, Debug)]
pub enum SentrixError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Parser error: {0}")]
    Parser(String),

    #[error("Graph error: {0}")]
    Graph(String),

    #[error("Git error: {0}")]
    Git(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("AI Engine error: {0}")]
    Ai(String),

    #[error("API Server error: {0}")]
    Api(String),

    #[error("General error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, SentrixError>;
