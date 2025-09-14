//main.rs
use anyhow::Result;
use clap::Parser;

mod cli;
mod crypto;
mod keystore;
mod types;
mod utils;
mod client;

use cli::Args;
use keystore::{
    generate_key_pair, 
    import_key_pair, 
    list_keys, 
    show_mnemonic, 
    load_key_store_from_path, 
    list_keys_from_loaded,
    show_mnemonic_from_loaded
};
use client::send_proof;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        cli::Commands::GenerateKey { name } => {
            generate_key_pair(&name)?;
        }
        cli::Commands::ListKeys => {
            list_keys()?;
        }
        cli::Commands::Send {
            proof_file,
            elf_file,
            key_name,
            proving_system,
            payload,
            game,
        } => {
            send_proof(
                args.endpoint,
                proof_file,
                elf_file,
                key_name,
                proving_system,
                payload,
                game,
            ).await?;
        }
        cli::Commands::ImportKey { name, mnemonic } => {
            import_key_pair(&name, &mnemonic)?;
        }
        cli::Commands::ShowMnemonic { name } => {
            show_mnemonic(&name)?;
        }
        cli::Commands::LoadKeyStore { path, subcommand } => {
            let key_store = load_key_store_from_path(&path)?;
            println!("Loaded key store from: {}", path);
            
            match subcommand {
                cli::LoadKeyStoreCommands::ListKeys => {
                    list_keys_from_loaded(&key_store)?;
                }
                cli::LoadKeyStoreCommands::ShowMnemonic { name } => {
                    show_mnemonic_from_loaded(&key_store, &name)?;
                }
            }
        }
    }

    Ok(())
}