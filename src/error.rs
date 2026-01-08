// error.rs

use thiserror::Error; // typed internal errors

#[derive(Error, Debug)]
pub enum TinyKVError {
    #[error("IO error")]
    IO(#[from] std::io::Error),
    #[error("Serialization error")]
    Serde(#[from] bincode::Error),
    #[error("Invalid state: {0}")]
    InvalidState(String),
}
