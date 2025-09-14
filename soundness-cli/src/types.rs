//types.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/// Supported zero-knowledge proving systems
/// Each system has different characteristics and use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ProvingSystem {
    #[value(help = "SP1 proving system - RISC-V based ZK-VM")]
    Sp1,
    #[value(help = "Ligetron proving system - High performance prover")]
    Ligetron,
    #[value(help = "RISC Zero proving system - General purpose ZK-VM")]
    Risc0,
    #[value(help = "Noir proving system - Domain specific language")]
    Noir,
    #[value(help = "StarkNet proving system - Ethereum L2 solution")]
    Starknet,
    #[value(help = "Miden VM proving system - Stack-based ZK-VM")]
    Miden,
}

impl std::fmt::Display for ProvingSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvingSystem::Sp1 => write!(f, "sp1"),
            ProvingSystem::Ligetron => write!(f, "ligetron"),
            ProvingSystem::Risc0 => write!(f, "risc0"),
            ProvingSystem::Noir => write!(f, "noir"),
            ProvingSystem::Starknet => write!(f, "starknet"),
            ProvingSystem::Miden => write!(f, "miden"),
        }
    }
}

/// Predefined game types for proof generation
/// Used when users don't want to provide custom ELF files
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Game {
    /// Classic 8-queens puzzle - place 8 queens on chessboard without attacking each other
    EightQueens,
    /// Tic-tac-toe game - strategic two-player game
    TicTacToe,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tic-tac-toe" | "tictactoe" => Ok(Game::TicTacToe),
            "8-queens" | "8queens" | "eight-queens" | "eightqueens" => Ok(Game::EightQueens),
            _ => Err(format!(
                "Invalid game: {}. Valid options: tic-tac-toe, tictactoe, 8-queens, 8queens, eight-queens, eightqueens", 
                s
            )),
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Game::EightQueens => write!(f, "8queens"),
            Game::TicTacToe => write!(f, "tictactoe"),
        }
    }
}

/// Represents a cryptographic key pair with public key and encrypted private key
/// The private key is always stored encrypted for security
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPair {
    /// Raw public key bytes (32 bytes for Ed25519)
    pub public_key: Vec<u8>,
    /// Base64-encoded public key string for display and transmission
    pub public_key_string: String,
    /// Encrypted private key - None only for legacy compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_secret_key: Option<EncryptedSecretKey>,
}

/// Encrypted secret key using AES-256-GCM with PBKDF2 key derivation
/// Provides confidentiality and authenticity protection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedSecretKey {
    /// Random salt for PBKDF2 key derivation (prevents rainbow table attacks)
    pub salt: Vec<u8>,
    /// Random nonce for AES-GCM encryption (ensures each encryption is unique)
    pub nonce: Vec<u8>,
    /// Encrypted secret key data with authentication tag
    pub encrypted_data: Vec<u8>,
    #[serde(default = "default_iterations")] 
    pub iterations: u32,
}

fn default_iterations() -> u32 {
    100_000  // OLD iterations
}

/// Local keystore containing multiple named key pairs
/// Stored as JSON file on disk with atomic write protection
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyStore {
    /// Map of key name to key pair data
    pub keys: HashMap<String, KeyPair>,
}

/// Server response from proof submission
/// Contains verification results and blockchain transaction details
#[derive(Debug, Deserialize)]
pub struct ServerResponse {
    /// Status of the submission ("success" or "error")
    pub status: String,
    /// Human-readable message describing the result
    pub message: String,
    /// The proving system that processed this proof
    pub proving_system: String,
    /// Whether the proof was cryptographically verified (true/false)
    pub proof_verification_status: Option<bool>,
    /// Status of the blockchain transaction submission
    pub sui_status: Option<String>,
    /// Blockchain transaction hash/digest
    pub sui_transaction_digest: Option<String>,
    /// Walrus blob ID for the stored proof data
    pub proof_data_blob_id: Option<String>,
    /// Walrus blob ID for the verification key/program
    pub vk_blob_id: Option<String>,
    /// Link to view transaction on blockchain explorer
    pub suiscan_link: Option<String>,
    /// Links to view stored data on Walrus explorer
    pub walruscan_links: Option<Vec<String>>,
}

// SECURITY CONSTANTS
// These values are carefully chosen for security vs performance balance

/// Length of salt for PBKDF2 key derivation (32 bytes = 256 bits)
/// Provides strong protection against rainbow table attacks
pub const SALT_LENGTH: usize = 32;

/// Length of nonce for AES-GCM encryption (12 bytes = 96 bits)
/// Standard nonce size for AES-GCM, provides 2^32 unique encryptions
pub const NONCE_LENGTH: usize = 12;

/// Length of derived encryption key (32 bytes = 256 bits)
// Matches AES-256 key size for maximum security
pub const KEY_LENGTH: usize = 32;

/// Number of PBKDF2 iterations for key derivation
//  iterations for PBKDF2-SHA256
/// This value balances security with reasonable performance
pub const ITERATIONS: u32 = 600_000;

// Minimum password length for user security
// Enforces reasonable password complexity
// pub const MIN_PASSWORD_LENGTH: usize = 8;

// BIP39 standard parameters
// pub const BIP39_SEED_LENGTH: usize = 64; // 512 bits
// pub const BIP39_ENTROPY_LENGTH: usize = 32; // 256 bits for 24-word mnemonic
// pub const BIP39_WORD_COUNT: usize = 24; // Standard word count for high entropy