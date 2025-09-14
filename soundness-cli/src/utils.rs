//utils.rs
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use crate::types::ServerResponse;

pub fn create_progress_bar(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(120));
    pb
}

/// Check if a string looks like a Walrus Blob ID vs a file path
/// Blob IDs are typically base64-like strings without path separators
pub fn is_blob_id(input: &str) -> bool {
    // Check if it contains path separators - if so, it's likely a file path
    if input.contains('/') || input.contains('\\') {
        return false;
    }

    // Check if it looks like a base64 string (alphanumeric + / and +, possibly with = padding)
    // and is reasonably long (blob IDs are typically 40+ characters)
    input.len() > 20
        && input.chars().all(|c| {
            c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=' || c == '-' || c == '_'
        })
}

pub fn format_server_response(response: &ServerResponse) {
    println!("\n🎯 Proof Submission Results");
    println!("═══════════════════════════");

    // Status and message
    let (icon, text) = if response.status == "success" {
        ("✅", response.status.to_uppercase())
    } else {
        ("❌", response.status.to_uppercase())
    };
    println!("{} Status: {}", icon, text);
    println!("📝 Message: {}", response.message);
    println!("🔧 Proving System: {}", response.proving_system);

    // Proof verification status
    if let Some(verification_status) = response.proof_verification_status {
        let (icon, text) = if verification_status {
            ("✅", "SUCCESS")
        } else {
            ("❌", "FAILED")
        };
        println!("🔍 Proof Verification: {} {}", icon, text);
    }

    // Sui transaction details
    if let Some(sui_status) = &response.sui_status {
        let sui_icon = if sui_status == "success" {
            "✅"
        } else {
            "❌"
        };
        println!(
            "⛓️  Sui Transaction: {} {}",
            sui_icon,
            sui_status.to_uppercase()
        );
    }

    if let Some(digest) = &response.sui_transaction_digest {
        println!("🔗 Transaction Digest: {}", digest);
    }

    // Blob IDs
    if let Some(proof_blob_id) = &response.proof_data_blob_id {
        println!("📦 Proof Blob ID: {}", proof_blob_id);
    }

    if let Some(vk_blob_id) = &response.vk_blob_id {
        println!("🔑 Program Blob ID: {}", vk_blob_id);
    }

    // Links
    if let Some(suiscan_link) = &response.suiscan_link {
        println!("🔍 Suiscan Link: {}", suiscan_link);
    }

    if let Some(walruscan_links) = &response.walruscan_links {
        if walruscan_links.len() >= 2 {
            println!("🌊 Walruscan Links:");
            println!("   📦 Proof Data: {}", walruscan_links[0]);
            println!("   🔑 VK: {}", walruscan_links[1]);
        }
    }

    println!("═══════════════════════════");
}