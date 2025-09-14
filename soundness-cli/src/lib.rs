//lib.rs
pub mod cli;
pub mod crypto;
pub mod keystore;
pub mod types;
pub mod utils;
pub mod client;

// Re-export commonly used types for convenience
pub use types::{ProvingSystem, Game, KeyStore, KeyPair, ServerResponse};
pub use keystore::{
    generate_key_pair, import_key_pair, list_keys, load_key_store, save_key_store,
    show_mnemonic, load_key_store_from_path, list_keys_from_loaded, show_mnemonic_from_loaded
};
pub use crypto::{
    sign_payload, get_public_key, encrypt_secret_key, decrypt_secret_key,
    decrypt_secret_key_to_mnemonic, get_encrypted_secret_key
};
pub use client::send_proof;
pub use utils::{create_progress_bar, is_blob_id, format_server_response};