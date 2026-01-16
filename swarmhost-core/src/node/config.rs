// node/config.rs - Configuration for Swarmhost nodes

use crate::crypto::{KeyPair, PlayerId};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for a Swarmhost node
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeConfig {
    /// Player's keypair (for signing actions)
    #[serde(skip)]
    pub keypair: Option<KeyPair>,

    /// Bootstrap server address for peer discovery
    pub bootstrap_server: Option<String>,

    /// Port to listen on for incoming connections
    pub listen_port: u16,

    /// Consensus configuration
    pub consensus: ConsensusConfig,

    /// Network configuration
    pub network: NetworkConfig,

    /// State management configuration
    pub state: StateConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Quorum size as a fraction (numerator/denominator)
    pub quorum_numerator: u32,
    pub quorum_denominator: u32,

    /// Enable optimistic execution?
    pub optimistic_execution: bool,

    /// Timeout for reaching consensus on an action
    #[serde(with = "serde_duration")]
    pub consensus_timeout: Duration,

    /// Maximum concurrent actions being validated
    pub max_concurrent_validations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Maximum number of peers to maintain connections with
    pub max_peers: usize,

    /// Heartbeat interval (ping peers to check they're alive)
    #[serde(with = "serde_duration")]
    pub heartbeat_interval: Duration,

    /// Peer timeout (disconnect if no heartbeat)
    #[serde(with = "serde_duration")]
    pub peer_timeout: Duration,

    /// Maximum message size in bytes
    pub max_message_size: usize,

    /// Enable message compression?
    pub enable_compression: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    /// How often to create state snapshots (in number of actions)
    pub snapshot_interval: u32,

    /// Maximum number of snapshots to keep in memory
    pub max_snapshots_in_memory: usize,

    /// Maximum action log size before requiring a snapshot
    pub max_action_log_size: usize,
}

// Helper module for Duration serialization
mod serde_duration {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            quorum_numerator: 2,
            quorum_denominator: 3,
            optimistic_execution: true,
            consensus_timeout: Duration::from_secs(5),
            max_concurrent_validations: 100,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            max_peers: 50,
            heartbeat_interval: Duration::from_secs(10),
            peer_timeout: Duration::from_secs(30),
            max_message_size: 1024 * 1024,
            enable_compression: true,
        }
    }
}

impl Default for StateConfig {
    fn default() -> Self {
        Self {
            snapshot_interval: 100,
            max_snapshots_in_memory: 10,
            max_action_log_size: 1000,
        }
    }
}

impl NodeConfig {
    /// Create a new config with a randomly generated keypair
    pub fn new() -> Self {
        Self {
            keypair: Some(KeyPair::generate()),
            ..Default::default()
        }
    }

    /// Create config with an existing keypair
    pub fn with_keypair(keypair: KeyPair) -> Self {
        Self {
            keypair: Some(keypair),
            ..Default::default()
        }
    }

    /// Get the player ID (public key)
    pub fn player_id(&self) -> Option<PlayerId> {
        self.keypair.as_ref().map(|kp| kp.public_key())
    }

    /// Set bootstrap server
    pub fn with_bootstrap(mut self, server: impl Into<String>) -> Self {
        self.bootstrap_server = Some(server.into());
        self
    }

    /// Set listen port
    pub fn with_port(mut self, port: u16) -> Self {
        self.listen_port = port;
        self
    }

    /// Enable/disable optimistic execution
    pub fn with_optimistic_execution(mut self, enabled: bool) -> Self {
        self.consensus.optimistic_execution = enabled;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.keypair.is_none() {
            return Err("Keypair must be set".to_string());
        }

        if self.consensus.quorum_numerator == 0 || self.consensus.quorum_denominator == 0 {
            return Err("Quorum fraction cannot have zero denominator/numerator".to_string());
        }

        if self.consensus.quorum_numerator > self.consensus.quorum_denominator {
            return Err("Quorum numerator cannot exceed denominator".to_string());
        }

        if self.network.max_message_size == 0 {
            return Err("Max message size must be > 0".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = NodeConfig::default();
        assert_eq!(config.consensus.quorum_numerator, 2);
        assert_eq!(config.consensus.quorum_denominator, 3);
        assert!(config.consensus.optimistic_execution);
    }

    #[test]
    fn test_new_config_generates_keypair() {
        let config = NodeConfig::new();
        assert!(config.keypair.is_some());
        assert!(config.player_id().is_some());
    }

    #[test]
    fn test_config_builder() {
        let config = NodeConfig::new()
            .with_bootstrap("localhost:8080")
            .with_port(9000)
            .with_optimistic_execution(false);

        assert_eq!(config.bootstrap_server, Some("localhost:8080".to_string()));
        assert_eq!(config.listen_port, 9000);
        assert!(!config.consensus.optimistic_execution);
    }

    #[test]
    fn test_validate_config() {
        let mut config = NodeConfig::new();
        assert!(config.validate().is_ok());

        // Invalid: no keypair
        config.keypair = None;
        assert!(config.validate().is_err());

        // Invalid: bad quorum
        config.keypair = Some(KeyPair::generate());
        config.consensus.quorum_numerator = 0;
        assert!(config.validate().is_err());
    }
}
