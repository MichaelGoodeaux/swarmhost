// lib.rs - Main entry point for Swarmhost core library

// Module declarations
pub mod consensus;
pub mod crypto;
pub mod error;
pub mod network;
pub mod node;
pub mod state;

// Re-export main types for convenience
pub use error::{Result, SwarmhostError};
pub use node::{NodeConfig, SwarmhostNode};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize logging for the library
pub fn init_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "swarmhost_core=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
