use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("placeholder error: {0}")]
    Placeholder(String),
}

pub type Result<T> = std::result::Result<T, Error>;
