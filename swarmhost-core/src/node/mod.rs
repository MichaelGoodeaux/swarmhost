// node/mod.rs - Main node implementation

mod config;

pub use config::{ConsensusConfig, NetworkConfig, NodeConfig, StateConfig};

use crate::crypto::PlayerId;
use crate::error::{Result, SwarmhostError};
use std::sync::Arc;
use tokio::sync::RwLock;

/// The main Swarmhost node
pub struct SwarmhostNode {
    config: NodeConfig,
    state: Arc<RwLock<NodeState>>,
}

/// Internal node state
struct NodeState {
    player_id: PlayerId,
    is_running: bool,
    connected_peers: Vec<PlayerId>,
}

impl SwarmhostNode {
    /// Create a new node with the given configuration
    pub fn new(config: NodeConfig) -> Result<Self> {
        config.validate().map_err(SwarmhostError::Config)?;

        let player_id = config
            .player_id()
            .ok_or_else(|| SwarmhostError::Config("No keypair set".to_string()))?;

        let state = Arc::new(RwLock::new(NodeState {
            player_id,
            is_running: false,
            connected_peers: Vec::new(),
        }));

        Ok(Self { config, state })
    }

    /// Get the player ID for this node
    pub async fn player_id(&self) -> PlayerId {
        let state = self.state.read().await;
        state.player_id
    }

    /// Start the node
    pub async fn start(&self) -> Result<()> {
        let mut state = self.state.write().await;

        if state.is_running {
            return Err(SwarmhostError::Node("Node already running".to_string()));
        }

        tracing::info!(
            "Starting Swarmhost node on port {}",
            self.config.listen_port
        );

        state.is_running = true;

        Ok(())
    }

    /// Stop the node
    pub async fn stop(&self) -> Result<()> {
        let mut state = self.state.write().await;

        if !state.is_running {
            return Ok(());
        }

        tracing::info!("Stopping Swarmhost node");

        state.is_running = false;
        state.connected_peers.clear();

        Ok(())
    }

    /// Check if the node is running
    pub async fn is_running(&self) -> bool {
        let state = self.state.read().await;
        state.is_running
    }

    /// Get the number of connected peers
    pub async fn peer_count(&self) -> usize {
        let state = self.state.read().await;
        state.connected_peers.len()
    }

    /// Join a game session
    pub async fn join_game(&self, _game_id: &str) -> Result<()> {
        let state = self.state.read().await;

        if !state.is_running {
            return Err(SwarmhostError::Node("Node not running".to_string()));
        }

        tracing::info!("Joining game: {}", _game_id);

        Ok(())
    }

    /// Submit an action to the network
    pub async fn submit_action(&self, _action_type: u32, _action_data: &[u8]) -> Result<()> {
        let state = self.state.read().await;

        if !state.is_running {
            return Err(SwarmhostError::Node("Node not running".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_creation() {
        let config = NodeConfig::new();
        let node = SwarmhostNode::new(config).unwrap();

        assert!(!node.is_running().await);
        assert_eq!(node.peer_count().await, 0);
    }

    #[tokio::test]
    async fn test_node_start_stop() {
        let config = NodeConfig::new();
        let node = SwarmhostNode::new(config).unwrap();

        node.start().await.unwrap();
        assert!(node.is_running().await);

        node.stop().await.unwrap();
        assert!(!node.is_running().await);
    }

    #[tokio::test]
    async fn test_double_start_fails() {
        let config = NodeConfig::new();
        let node = SwarmhostNode::new(config).unwrap();

        node.start().await.unwrap();
        let result = node.start().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_submit_action_when_not_running() {
        let config = NodeConfig::new();
        let node = SwarmhostNode::new(config).unwrap();

        let result = node.submit_action(1, b"test").await;
        assert!(result.is_err());
    }
}
