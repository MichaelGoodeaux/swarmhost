// crypto/mod.rs - Cryptographic primitives

use crate::error::{Result, SwarmhostError};
use blake2::{Blake2s256, Digest};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

pub type Hash = [u8; 32];
pub type PlayerId = [u8; 32]; // Public key = player ID

/// Keypair for signing and verification
#[derive(Clone, Debug)]
pub struct KeyPair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl KeyPair {
    /// Generate a new random keypair
    pub fn generate() -> Self {
        let secret_bytes: [u8; 32] = rand::random();
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let verifying_key = signing_key.verifying_key();

        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Create from existing private key bytes
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self> {
        let signing_key = SigningKey::from_bytes(bytes);
        let verifying_key = signing_key.verifying_key();

        Ok(Self {
            signing_key,
            verifying_key,
        })
    }

    /// Get the public key (player ID)
    pub fn public_key(&self) -> PlayerId {
        self.verifying_key.to_bytes()
    }

    /// Get the private key bytes (for storage/serialization)
    pub fn private_key(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.signing_key.sign(message).to_bytes().to_vec()
    }

    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
        let sig = Signature::from_slice(signature)
            .map_err(|e| SwarmhostError::crypto(format!("Invalid signature: {}", e)))?;

        self.verifying_key
            .verify(message, &sig)
            .map_err(|e| SwarmhostError::crypto(format!("Verification failed: {}", e)))
    }
}

/// Verify a signature with a public key
pub fn verify_signature(public_key: &PlayerId, message: &[u8], signature: &[u8]) -> Result<()> {
    let verifying_key = VerifyingKey::from_bytes(public_key)
        .map_err(|e| SwarmhostError::crypto(format!("Invalid public key: {}", e)))?;

    let sig = Signature::from_slice(signature)
        .map_err(|e| SwarmhostError::crypto(format!("Invalid signature: {}", e)))?;

    verifying_key
        .verify(message, &sig)
        .map_err(|e| SwarmhostError::crypto(format!("Verification failed: {}", e)))
}

/// Hash data using Blake2s
pub fn hash(data: &[u8]) -> Hash {
    let mut hasher = Blake2s256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Hash multiple pieces of data together
pub fn hash_multiple(data_pieces: &[&[u8]]) -> Hash {
    let mut hasher = Blake2s256::new();
    for piece in data_pieces {
        hasher.update(piece);
    }
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = KeyPair::generate();
        let public_key = keypair.public_key();
        assert_eq!(public_key.len(), 32);
    }

    #[test]
    fn test_sign_and_verify() {
        let keypair = KeyPair::generate();
        let message = b"Hello, Swarmhost!";

        let signature = keypair.sign(message);
        assert!(keypair.verify(message, &signature).is_ok());

        // Wrong message should fail
        let wrong_message = b"Wrong message";
        assert!(keypair.verify(wrong_message, &signature).is_err());
    }

    #[test]
    fn test_verify_with_public_key() {
        let keypair = KeyPair::generate();
        let public_key = keypair.public_key();
        let message = b"Test message";
        let signature = keypair.sign(message);

        assert!(verify_signature(&public_key, message, &signature).is_ok());
    }

    #[test]
    fn test_hash() {
        let data = b"Some data to hash";
        let hash1 = hash(data);
        let hash2 = hash(data);

        // Same data should produce same hash
        assert_eq!(hash1, hash2);

        // Different data should produce different hash
        let different_data = b"Different data";
        let hash3 = hash(different_data);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_hash_multiple() {
        let piece1 = b"piece1";
        let piece2 = b"piece2";
        let piece3 = b"piece3";

        let hash1 = hash_multiple(&[piece1, piece2, piece3]);
        let hash2 = hash_multiple(&[piece1, piece2, piece3]);

        assert_eq!(hash1, hash2);

        // Different order should produce different hash
        let hash3 = hash_multiple(&[piece3, piece2, piece1]);
        assert_ne!(hash1, hash3);
    }
}
