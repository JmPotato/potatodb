use std::result;

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Internal Error: {0}")]
    Config(String),
}

pub type Result<T> = result::Result<T, Error>;
