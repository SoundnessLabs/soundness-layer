//crypto.rs
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use bip39::Mnemonic;
use anyhow::Result;
use ed25519_dalek::{Signer, SigningKey};
use once_cell::sync::Lazy;
use pbkdf2::pbkdf2_hmac_array;
use rand::{rngs::OsRng, RngCore};
use sha2::{Digest, Sha256};
use std::sync::Mutex;
use rpassword::prompt_password;

use crate::types::{EncryptedSecretKey, KeyStore, SALT_LENGTH, NONCE_LENGTH, KEY_LENGTH, ITERATIONS};
use crate::keystore::load_key_store;

static PASSWORD_CACHE: Lazy<Mutex<Option<(String, String)>>> = Lazy::new(|| Mutex::new(None));

pub fn encrypt_secret_key(secret_key: &[u8], password: &str) -> Result<EncryptedSecretKey> {
    let mut rng = OsRng;
    let mut salt = [0u8; SALT_LENGTH];
    let mut nonce = [0u8; NONCE_LENGTH];
    rng.fill_bytes(&mut salt);
    rng.fill_bytes(&mut nonce);

    let key_bytes = derive_key(password, &salt, ITERATIONS);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let encrypted_data = cipher
        .encrypt(Nonce::from_slice(&nonce), secret_key)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    Ok(EncryptedSecretKey {
        salt: salt.to_vec(),
        nonce: nonce.to_vec(),
        encrypted_data,
        iterations: ITERATIONS,
    })
}

pub fn derive_key(password: &str, salt: &[u8], iterations: u32) -> [u8; KEY_LENGTH] {
    pbkdf2_hmac_array::<Sha256, KEY_LENGTH>(password.as_bytes(), salt, iterations)
}

pub fn decrypt_secret_key(encrypted: &EncryptedSecretKey, password: &str) -> Result<Vec<u8>> {
    let key_bytes = derive_key(password, &encrypted.salt, encrypted.iterations);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    cipher
        .decrypt(
            Nonce::from_slice(&encrypted.nonce),
            encrypted.encrypted_data.as_slice(),
        )
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))
}

pub fn decrypt_secret_key_to_mnemonic(encrypted: &EncryptedSecretKey, password: &str) -> Result<String> {
    let secret_key_bytes = decrypt_secret_key(encrypted, password)?;
    
    // Convert secret key bytes to mnemonic
    let mnemonic = Mnemonic::from_entropy(&secret_key_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to create mnemonic from secret key: {}", e))?;
    
    Ok(mnemonic.to_string())
}

pub fn get_encrypted_secret_key(key_name: &str) -> Result<EncryptedSecretKey> {
    let key_store = load_key_store()?;
    let key_pair = key_store
        .keys
        .get(key_name)
        .ok_or_else(|| anyhow::anyhow!("Key pair '{}' not found", key_name))?;

    key_pair
        .encrypted_secret_key
        .as_ref()
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Secret key not found for '{}'", key_name))
}

// Calculate hash of key store contents
fn calculate_key_store_hash(key_store: &KeyStore) -> String {
    let serialized = serde_json::to_string(key_store).unwrap_or_default();
    format!("{:x}", Sha256::digest(serialized.as_bytes()))
}

pub fn sign_payload(payload: &[u8], key_name: &str) -> Result<Vec<u8>> {
    use crate::utils::create_progress_bar;
    
    let key_store = load_key_store()?;
    let key_store_hash = calculate_key_store_hash(&key_store);

    let key_pair = key_store
        .keys
        .get(key_name)
        .ok_or_else(|| anyhow::anyhow!("Key pair '{}' not found", key_name))?;

    let encrypted_secret = key_pair
        .encrypted_secret_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Secret key not found for '{}'", key_name))?;

    // Create a new scope for the password guard to ensure it's dropped properly
    let password = {
        let mut password_guard = PASSWORD_CACHE.lock().unwrap();

        if let Some((stored_password, stored_hash)) = password_guard.as_ref() {
            // Check if key store has changed
            if stored_hash != &key_store_hash {
                *password_guard = None;
                drop(password_guard);
                return sign_payload(payload, key_name);
            }
            stored_password.clone()
        } else {
            // If no password is stored, prompt for it
            let new_password = prompt_password("Enter password to decrypt the secret key: ")
                .map_err(|e| anyhow::anyhow!("Failed to read password: {}", e))?;

            // Try to decrypt with the password to verify it's correct
            if let Err(e) = decrypt_secret_key(encrypted_secret, &new_password) {
                anyhow::bail!("Invalid password: {}", e);
            }

            // Store the password and key store hash
            *password_guard = Some((new_password.clone(), key_store_hash));
            new_password
        }
    }; // password_guard is dropped here

    // Only show the progress bar after we have the password
    let pb = create_progress_bar("✍️  Signing payload...");

    let secret_key_bytes = decrypt_secret_key(encrypted_secret, &password)?;
    let secret_key_array: [u8; 32] = secret_key_bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid secret key length"))?;

    let signing_key = SigningKey::from_bytes(&secret_key_array);
    let signature = signing_key.sign(payload);
    pb.finish_with_message("✍️  Payload signed successfully");

    Ok(signature.to_bytes().to_vec())
}

pub fn get_public_key(key_name: &str) -> Result<Vec<u8>> {
    let key_store = load_key_store()?;
    let key_pair = key_store
        .keys
        .get(key_name)
        .ok_or_else(|| anyhow::anyhow!("Key pair '{}' not found", key_name))?;
    Ok(key_pair.public_key.clone())
}