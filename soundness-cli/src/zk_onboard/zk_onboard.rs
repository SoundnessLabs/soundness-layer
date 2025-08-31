use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZkProof {
    pub username: String,
    pub nonce: String,
    pub proof: String,
}

pub struct ZkOnboard;

impl ZkOnboard {
    pub fn new() -> Self {
        ZkOnboard
    }

    /// Register a user and produce a mock ZK-proof (deterministic hash over username+nonce).
    /// This is a *mock* proof intended for onboarding PoC only.
    pub fn register_user(&self, username: &str) -> Result<ZkProof, String> {
        if username.trim().is_empty() {
            return Err("username empty".into());
        }
        let nonce = rand::thread_rng().gen::<u64>().to_string();
        let mut hasher = Sha256::new();
        hasher.update(username.as_bytes());
        hasher.update(b":");
        hasher.update(nonce.as_bytes());
        let digest = hasher.finalize();
        let proof_hex = hex::encode(digest);
        Ok(ZkProof {
            username: username.to_string(),
            nonce,
            proof: proof_hex,
        })
    }

    /// Verify the mock proof by recomputing the hash.
    pub fn verify_proof(&self, proof: &ZkProof) -> Result<bool, String> {
        if proof.username.trim().is_empty() {
            return Err("invalid proof: empty username".into());
        }
        let mut hasher = Sha256::new();
        hasher.update(proof.username.as_bytes());
        hasher.update(b":");
        hasher.update(proof.nonce.as_bytes());
        let digest = hasher.finalize();
        let expected = hex::encode(digest);
        Ok(expected == proof.proof)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_register_and_verify() {
        let zk = ZkOnboard::new();
        let p = zk.register_user("alice").expect("register failed");
        assert!(zk.verify_proof(&p).unwrap());
    }
}
