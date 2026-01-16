// error.rs - Error types for Swarmhost

use thiserror::Error;

pub type Result<T> = std::result::Result<T, SwarmhostError>;

#[derive(Error, Debug)]
pub enum SwarmhostError {
    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    #[error("Consensus error: {0}")]
    Consensus(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Cryptography error: {0}")]
    Crypto(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Node error: {0}")]
    Node(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Peer error: {0}")]
    Peer(String),

    #[error("Configuration error: {0}")]
    Config(String),
}

// Helper for creating errors
impl SwarmhostError {
    pub fn consensus(msg: impl Into<String>) -> Self {
        SwarmhostError::Consensus(msg.into())
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        SwarmhostError::Validation(msg.into())
    }

    pub fn crypto(msg: impl Into<String>) -> Self {
        SwarmhostError::Crypto(msg.into())
    }

    pub fn node(msg: impl Into<String>) -> Self {
        SwarmhostError::Node(msg.into())
    }

    pub fn timeout(msg: impl Into<String>) -> Self {
        SwarmhostError::Timeout(msg.into())
    }
}
