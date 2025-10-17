use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("YAML parsing error: {0}")]
    Yaml(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Template error: {0}")]
    Template(String),
    
    #[error("Markdown error: {0}")]
    Markdown(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::Yaml(err.to_string())
    }
}

