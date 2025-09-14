//cli.rs
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::types::{ProvingSystem, Game};

#[derive(Parser, Debug)]
#[command(
    name = "soundness-cli",
    author = "Soundness Layer Team",
    version = "1.0.0",
    about = "A CLI tool for sending zero-knowledge proofs to Soundness Layer",
    long_about = "Soundness CLI is a command-line tool for interacting with the Soundness Layer.\n\nKey features:\n- Generate and manage Ed25519 key pairs\n- Submit zero-knowledge proofs from various proving systems\n- Support for local files and Walrus blob storage\n- Secure key storage with AES-256-GCM encryption\n\nSupported proving systems: 
    * SP1
    * Ligetron
    * RISC0
    * Noir
    * StarkNet
    * Miden
    \n\nFor detailed help on any command, use: soundness-cli <command> --help"
)]
pub struct Args {
    #[arg(
        short, 
        long, 
        default_value = "https://testnet.soundness.xyz",
        help = "Soundness Layer endpoint URL",
        long_help = "The base URL of the Soundness Layer API endpoint.\nDefaults to the testnet endpoint. Change this for testnet or local development.",
        value_name = "URL"
    )]
    pub endpoint: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        about = "Generate a new cryptographic key pair",
        long_about = "Generate a new Ed25519 key pair with mnemonic phrase for signing proofs.\nThe private key is encrypted with AES-256-GCM and stored locally.\nSave the mnemonic phrase securely as it's needed for key recovery."
    )]
    GenerateKey {
        #[arg(
            short, 
            long,
            help = "Name for the key pair (used as identifier)",
            value_name = "KEY_NAME"
        )]
        name: String,
    },

    #[command(
        about = "List all stored key pairs",
        long_about = "Display all key pairs stored in the local keystore with their public keys.\nPrivate keys are encrypted and not displayed for security."
    )]
    ListKeys,

    #[command(
        about = "Send a proof to the Soundness Layer",
        long_about = "Submit a zero-knowledge proof to the Soundness Layer for verification.\nSupports multiple proving systems and can handle both local files and Walrus blob IDs.\n\nExamples:\n  soundness-cli send -p proof.bin -g tic-tac-toe -k my_key -s sp1\n  soundness-cli send -p proof.bin -l program.elf -k my_key -s risc0\n  soundness-cli send -p blob_id_123 -g 8-queens -k my_key -s noir"
    )]
    Send {
        #[arg(
            short = 'p', 
            long,
            help = "Path to proof file or Walrus blob ID",
            long_help = "Path to the zero-knowledge proof file (e.g., proof.bin) or a Walrus blob ID.\nThe tool automatically detects whether the input is a file path or blob ID.",
            value_name = "PROOF_FILE_OR_BLOB_ID"
        )]
        proof_file: PathBuf,

        #[arg(
            short = 'l', 
            long,
            help = "Path to ELF file or Walrus blob ID (optional)",
            long_help = "Path to the compiled program ELF file or Walrus blob ID.\nRequired when not using a predefined game. Optional when using --game flag.",
            value_name = "ELF_FILE_OR_BLOB_ID"
        )]
        elf_file: Option<PathBuf>,

        #[arg(
            short = 'k', 
            long,
            help = "Name of the key pair to use for signing",
            long_help = "Name of the key pair (created with generate-key) to use for signing the proof.\nUse 'list-keys' to see available key pairs.",
            value_name = "KEY_NAME"
        )]
        key_name: String,

        #[arg(
            short = 's', 
            long,
            help = "Zero-knowledge proving system used",
            long_help = "The proving system used to generate the proof.\nSupported systems: sp1, ligetron, risc0, noir, starknet, miden",
            value_enum
        )]
        proving_system: ProvingSystem,

        #[arg(
            short = 'd', 
            long,
            help = "Additional JSON payload (optional)",
            long_help = "Optional JSON payload to include with the proof submission.\nUseful for providing additional context or metadata.",
            value_name = "JSON_PAYLOAD"
        )]
        payload: Option<serde_json::Value>,

        #[arg(
            short = 'g', 
            long, 
            value_parser = clap::value_parser!(Game),
            help = "Predefined game type (alternative to ELF file)",
            long_help = "Use a predefined game instead of providing an ELF file.\nAvailable games:\n  - tic-tac-toe, tictactoe: Tic-tac-toe game\n  - 8-queens, 8queens, eight-queens, eightqueens: Eight queens puzzle",
            value_name = "GAME_TYPE"
        )]
        game: Option<Game>,
    },

    #[command(
        about = "Import an existing key pair from mnemonic",
        long_about = "Import an existing key pair using a BIP39 mnemonic phrase.\nUseful for recovering keys or using keys generated elsewhere.\nThe imported private key will be encrypted and stored in the local keystore."
    )]
    ImportKey {
        #[arg(
            short, 
            long,
            help = "Name for the imported key pair",
            value_name = "KEY_NAME"
        )]
        name: String,
        
        #[arg(
            short, 
            long,
            help = "BIP39 mnemonic phrase (24 words)",
            long_help = "BIP39 mnemonic phrase used to restore the key pair.\nShould be 24 words separated by spaces.",
            value_name = "MNEMONIC_PHRASE"
        )]
        mnemonic: String,
    },
        #[command(
        about = "Show mnemonic phrase for a key pair",
        long_about = "Display the mnemonic phrase for an existing key pair.\nRequires the password used to encrypt the secret key."
    )]

    ShowMnemonic {
        #[arg(
            long,
            short = 'n',
            help = "Name of the key pair",
            value_name = "KEY_NAME"
        )]
        name: String,
    },

    #[command(
        about = "Load key store from specific file",
        long_about = "Load and use a key store from a specific JSON file path.\nUseful for working with multiple key stores."
    )]
    LoadKeyStore {
        #[arg(
            long,
            short = 'p',
            help = "Path to the key store JSON file",
            value_name = "KEY_STORE_PATH"
        )]
        path: String,
        
        #[command(subcommand)]
        subcommand: LoadKeyStoreCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum LoadKeyStoreCommands {
    #[command(about = "List keys in the loaded key store")]
    ListKeys,
    
    #[command(about = "Show mnemonic from loaded key store")]
    ShowMnemonic {
        #[arg(help = "Name of the key pair", value_name = "KEY_NAME")]
        name: String,
    },
}