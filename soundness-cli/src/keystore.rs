use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use bip39;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use rpassword::prompt_password;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use crate::types::{KeyPair, KeyStore};
use crate::crypto::{encrypt_secret_key, decrypt_secret_key_to_mnemonic, get_encrypted_secret_key};

pub fn load_key_store() -> Result<KeyStore> {
    let key_store_path = PathBuf::from("key_store.json");
    if key_store_path.exists() {
        let contents = fs::read_to_string(&key_store_path)?;
        let key_store: KeyStore = serde_json::from_str(&contents)?;
        Ok(key_store)
    } else {
        Ok(KeyStore {
            keys: std::collections::HashMap::new(),
        })
    }
}

pub fn save_key_store(key_store: &KeyStore) -> Result<()> {
    let key_store_path = PathBuf::from("key_store.json");
    let contents = serde_json::to_string_pretty(key_store)?;
    fs::write(key_store_path, contents)?;
    Ok(())
}

pub fn show_mnemonic(key_name: &str) -> Result<()> {
    let key_store = load_key_store()?;
    
    if !key_store.keys.contains_key(key_name) {
        anyhow::bail!("Key pair with name '{}' not found", key_name);
    }

    // Get encrypted secret key
    let encrypted_secret = get_encrypted_secret_key(key_name)?;

    // Prompt for password
    let password = prompt_password("Enter password to decrypt the secret key: ")
        .map_err(|e| anyhow::anyhow!("Failed to read password: {}", e))?;

    // Decrypt and get mnemonic
    let mnemonic = decrypt_secret_key_to_mnemonic(&encrypted_secret, &password)?;

    println!("\nMnemonic phrase for key '{}':", key_name);
    println!("{}", mnemonic);
    println!("\n⚠️  Keep this mnemonic secure! It can be used to recover your key.");

    Ok(())
}

pub fn load_key_store_from_path(path: &str) -> Result<KeyStore> {
    let key_store_path = PathBuf::from(path);
    if key_store_path.exists() {
        let contents = fs::read_to_string(&key_store_path)?;
        let key_store: KeyStore = serde_json::from_str(&contents)?;
        Ok(key_store)
    } else {
        anyhow::bail!("Key store file not found: {}", path)
    }
}

pub fn list_keys_from_loaded(key_store: &KeyStore) -> Result<()> {
    if key_store.keys.is_empty() {
        println!("No key pairs found in the loaded key store.");
        return Ok(());
    }

    println!("Available key pairs:");
    for (name, key_pair) in &key_store.keys {
        println!("- {} (Public key: {})", name, key_pair.public_key_string);
    }
    Ok(())
}

pub fn show_mnemonic_from_loaded(key_store: &KeyStore, key_name: &str) -> Result<()> {
    let key_pair = key_store
        .keys
        .get(key_name)
        .ok_or_else(|| anyhow::anyhow!("Key pair '{}' not found", key_name))?;

    let encrypted_secret = key_pair
        .encrypted_secret_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Secret key not found for '{}'", key_name))?;

    let password = prompt_password("Enter password to decrypt the secret key: ")
        .map_err(|e| anyhow::anyhow!("Failed to read password: {}", e))?;

    let mnemonic = decrypt_secret_key_to_mnemonic(encrypted_secret, &password)?;

    println!("\nMnemonic phrase for key '{}':", key_name);
    println!("{}", mnemonic);
    println!("\n⚠️  Keep this mnemonic secure!");

    Ok(())
}


pub fn generate_key_pair(name: &str) -> Result<()> {
    let mut key_store = load_key_store()?;

    if key_store.keys.contains_key(name) {
        anyhow::bail!("Key pair with name '{}' already exists", name);
    }

    // Generate a new key pair
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();
    let public_key_bytes = verifying_key.to_bytes();
    let public_key_string = BASE64.encode(&public_key_bytes);

    // Generate mnemonic from secret key
    let secret_key_bytes = signing_key.to_bytes();
    let mnemonic = bip39::Mnemonic::from_entropy(&secret_key_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to generate mnemonic: {}", e))?;
    let mnemonic_string = mnemonic.to_string();

    println!("\n📝 IMPORTANT: Save this mnemonic phrase securely!");
    println!("{}", mnemonic_string);
    println!("⚠️  WARNING: This is the only time you'll see this mnemonic!");
    println!("⚠️  WARNING: You'll need it to recover your secret key if the key store is lost!");

    // Get password for secret key encryption
    let password = prompt_password("\nEnter password for secret key: ")
        .map_err(|e| anyhow::anyhow!("Failed to read password: {}", e))?;
    let confirm_password = prompt_password("Confirm password: ")
        .map_err(|e| anyhow::anyhow!("Failed to read password: {}", e))?;

    if password != confirm_password {
        anyhow::bail!("Passwords do not match");
    }

    // Encrypt the secret key
    let encrypted_secret = encrypt_secret_key(&secret_key_bytes, &password)?;

    // Save the key pair
    key_store.keys.insert(
        name.to_string(),
        KeyPair {
            public_key: public_key_bytes.to_vec(),
            public_key_string: public_key_string.clone(),
            encrypted_secret_key: Some(encrypted_secret),
        },
    );

    save_key_store(&key_store)?;
    println!("\n✅ Generated new key pair '{}'", name);
    println!("🔑 Public key: {}", public_key_string);
    Ok(())
}

pub fn import_key_pair(name: &str, mnemonic: &str) -> Result<()> {
    let mut key_store = load_key_store()?;

    if key_store.keys.contains_key(name) {
        anyhow::bail!("Key pair with name '{}' already exists", name);
    }

    // Convert mnemonic to entropy
    let mnemonic = bip39::Mnemonic::from_str(mnemonic)
        .map_err(|e| anyhow::anyhow!("Invalid mnemonic: {}", e))?;
    let entropy = mnemonic.to_entropy();

    // Create secret key from entropy
    let secret_key_array: [u8; 32] = entropy
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid entropy length"))?;
    let signing_key = SigningKey::from_bytes(&secret_key_array);
    let verifying_key = signing_key.verifying_key();
    let public_key_bytes = verifying_key.to_bytes();
    let public_key_string = BASE64.encode(&public_key_bytes);

    // Get password for secret key encryption
    let password = prompt_password("\nEnter password for secret key: ")
        .map_err(|e| anyhow::anyhow!("Failed to read password: {}", e))?;
    let confirm_password = prompt_password("Confirm password: ")
        .map_err(|e| anyhow::anyhow!("Failed to read password: {}", e))?;

    if password != confirm_password {
        anyhow::bail!("Passwords do not match");
    }

    // Encrypt the secret key
    let encrypted_secret = encrypt_secret_key(&secret_key_array, &password)?;

    // Save the key pair
    key_store.keys.insert(
        name.to_string(),
        KeyPair {
            public_key: public_key_bytes.to_vec(),
            public_key_string: public_key_string.clone(),
            encrypted_secret_key: Some(encrypted_secret),
        },
    );

    save_key_store(&key_store)?;
    println!("\n✅ Imported key pair '{}'", name);
    println!("🔑 Public key: {}", public_key_string);
    Ok(())
}

pub fn list_keys() -> Result<()> {
    let key_store = load_key_store()?;

    if key_store.keys.is_empty() {
        println!("No key pairs found. Generate one with 'generate-key' command.");
        return Ok(());
    }

    println!("Available key pairs:");
    for (name, key_pair) in key_store.keys {
        println!("- {} (Public key: {})", name, key_pair.public_key_string);
    }
    Ok(())
}