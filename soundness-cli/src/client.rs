//client.rs
use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use std::fs;
use std::path::PathBuf;

use crate::crypto::{sign_payload, get_public_key};
use crate::types::{ProvingSystem, Game, ServerResponse};
use crate::utils::{create_progress_bar, is_blob_id, format_server_response};

pub async fn send_proof(
    endpoint: String,
    proof_file: PathBuf,
    elf_file: Option<PathBuf>,
    key_name: String,
    proving_system: ProvingSystem,
    payload: Option<serde_json::Value>,
    game: Option<Game>,
) -> Result<()> {
    // Validate that either a game flag or ELF file is provided
    if game.is_none() && elf_file.is_none() {
        anyhow::bail!("❌ Error: When no game flag is provided, an ELF file (-l/--elf-file) must be specified.\n\nUsage:\n  - With game: soundness-cli send -p proof.bin -g tic-tac-toe -k my_key\n  - With ELF:  soundness-cli send -p proof.bin -l program.elf -k my_key");
    }

    let proof_input = proof_file.to_string_lossy().to_string();
    let proof_is_blob = is_blob_id(&proof_input);

    let (elf_input, elf_is_blob) = if let Some(ref elf_path) = elf_file {
        let elf_input = elf_path.to_string_lossy().to_string();
        let elf_is_blob = is_blob_id(&elf_input);
        (Some(elf_input), elf_is_blob)
    } else {
        (None, false)
    };

    // Print the inputs to the user
    println!("🔍 Analyzing inputs...");
    if proof_is_blob {
        println!("📁 Proof: Detected as Walrus Blob ID: {}", proof_input);
    } else {
        println!("📁 Proof: Detected as file path: {}", proof_input);
    }

    match &elf_input {
        Some(elf_str) => {
            if elf_is_blob {
                println!("📁 ELF Program: Detected as Walrus Blob ID: {}", elf_str);
            } else {
                println!("📁 ELF Program: Detected as file path: {}", elf_str);
            }
        }
        None => {
            println!("📁 ELF Program: Not provided (using game mode)");
        }
    }

    let reading_pb = create_progress_bar("📂 Processing inputs...");

    // Handle proof input
    let (proof_content, proof_blob_id, proof_filename) = if proof_is_blob {
        (None, Some(proof_input.clone()), "proof.bin".to_string())
    } else {
        let content = fs::read(&proof_file).with_context(|| {
            format!("Failed to read proof file: {}", proof_file.display())
        })?;
        let filename = proof_file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        (Some(BASE64.encode(&content)), None, filename)
    };

    // Handle optional ELF input
    let (elf_content, elf_blob_id, elf_filename) = match (&elf_input, &elf_file) {
        (Some(elf_str), Some(elf_path)) => {
            if elf_is_blob {
                (None, Some(elf_str.clone()), "program.elf".to_string())
            } else {
                let content = fs::read(elf_path).with_context(|| {
                    format!("Failed to read ELF file: {}", elf_path.display())
                })?;
                let filename = elf_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                (Some(BASE64.encode(&content)), None, filename)
            }
        }
        _ => (None, None, "".to_string()),
    };

    reading_pb.finish_with_message("📂 Inputs processed successfully");

    // Create the request body with support for both file content and blob IDs
    let mut request_body = serde_json::json!({
        "proof_filename": proof_filename,
        "proving_system": format!("{:?}", proving_system).to_lowercase(),
        "payload": payload.unwrap_or_default(),
        "game": game.unwrap_or(Game::EightQueens).to_string(),
    });

    // Add proof data (either content or blob ID)
    if let Some(content) = proof_content {
        request_body["proof"] = serde_json::Value::String(content.clone());
    } else if let Some(blob_id) = proof_blob_id {
        request_body["proof_blob_id"] = serde_json::Value::String(blob_id);
    }

    // Add ELF data only if provided (either content or blob ID)
    if !elf_filename.is_empty() {
        request_body["elf_filename"] = serde_json::Value::String(elf_filename.clone());

        if let Some(content) = elf_content {
            request_body["elf"] = serde_json::Value::String(content.clone());
        } else if let Some(blob_id) = elf_blob_id {
            request_body["elf_blob_id"] = serde_json::Value::String(blob_id);
        }
    }

    // Create canonical string for signing
    let proof_value = request_body
        .get("proof")
        .or_else(|| request_body.get("proof_blob_id"))
        .unwrap_or(&serde_json::Value::Null)
        .as_str()
        .unwrap_or("");

    let elf_value = request_body
        .get("elf")
        .or_else(|| request_body.get("elf_blob_id"))
        .unwrap_or(&serde_json::Value::Null)
        .as_str()
        .unwrap_or("");

    let canonical_string = if !elf_filename.is_empty() {
        format!(
            "proof:{}\nelf:{}\nproof_filename:{}\nelf_filename:{}\nproving_system:{}",
            proof_value,
            elf_value,
            proof_filename,
            elf_filename,
            format!("{:?}", proving_system).to_lowercase()
        )
    } else {
        format!(
            "proof:{}\nproof_filename:{}\nproving_system:{}",
            proof_value,
            proof_filename,
            format!("{:?}", proving_system).to_lowercase()
        )
    };

    request_body["canonical_string"] = serde_json::Value::String(canonical_string.clone());

    // Sign the canonical string
    let signature = sign_payload(canonical_string.as_bytes(), &key_name)?;
    let public_key = get_public_key(&key_name)?;

    // Send the request
    let sending_pb = create_progress_bar("🚀 Sending to server...");
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/api/proof", endpoint))
        .header("Content-Type", "application/json")
        .header("X-Signature", BASE64.encode(&signature))
        .header("X-Public-Key", BASE64.encode(&public_key))
        .json(&request_body)
        .send()
        .await
        .with_context(|| format!("Failed to send request to {}", endpoint))?;

    sending_pb.finish_with_message("🚀 Request sent successfully");

    // Check if the request was successful
    if response.status().is_success() {
        println!("\n✅ Successfully sent files to {}", endpoint);
        let response_text = response.text().await?;

        // Try to parse and format the response
        match serde_json::from_str::<ServerResponse>(&response_text) {
            Ok(parsed_response) => {
                format_server_response(&parsed_response);
            }
            Err(_) => {
                // Fallback to raw JSON if parsing fails
                println!("📄 Raw server response:");
                println!("{}", response_text);
            }
        }
    } else {
        println!("\n❌ Error: Server returned status {}", response.status());
        let error_text = response.text().await?;
        println!("Error details: {}", error_text);
    }

    Ok(())
}