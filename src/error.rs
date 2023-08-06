use thiserror::Error;

#[derive(Error, Debug)]
pub enum XmlRpcError {
    #[error("IoError: {0}")]
    Io(#[from] std::io::Error),
    #[error("Not Found: {0}")]
    NotFound(String),
}
