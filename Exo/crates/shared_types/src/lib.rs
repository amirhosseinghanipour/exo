use thiserror::Error;
pub use url::Url;

#[derive(Error, Debug, Clone)]
pub enum ExoError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("URL parsing error: {0}")]
    UrlParse(String),

    #[error("Browser core error: {0}")]
    Core(String),

    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, ExoError>;
