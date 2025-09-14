# Soundness CLI

Command-line interface tool for interacting with the Soundness Layer. The CLI provides secure key management, zero-knowledge proof submission, and comprehensive blockchain integration capabilities.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Architecture](#architecture)
- [Usage](#usage)
- [Command Reference](#command-reference)
- [Security](#security)
- [Configuration](#configuration)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

## Features

### Core Capabilities
- **Cryptographic Key Management**: Generate, import, and manage Ed25519 key pairs with secure local storage
- **Zero-Knowledge Proof Submission**: Submit proofs from multiple proving systems to the Soundness Layer
- **Multi-Storage Support**: Handle both local files and Walrus distributed storage blob IDs
- **Blockchain Integration**: Direct integration with Sui blockchain for on-chain proof verification
- **Security-First Design**: AES-256-GCM encryption with PBKDF2 key derivation

### Supported Proving Systems
- **SP1**: RISC-V based ZK-VM for general-purpose computation
- **Ligetron**: High-performance proving system optimized for speed
- **RISC Zero**: General-purpose zero-knowledge virtual machine
- **Noir**: Domain-specific language for zero-knowledge proofs
- **StarkNet**: Ethereum Layer 2 scaling solution with STARK proofs
- **Miden VM**: Stack-based zero-knowledge virtual machine

### Predefined Games
- **Eight Queens Puzzle**: Classic constraint satisfaction problem
- **Tic-Tac-Toe**: Strategic two-player game implementation

## Installation

### Prerequisites
- Rust toolchain (1.70.0 or later)
- Git (for source installation)

### Method 1: Quick Install via soundnessup (Recommended)

The `soundnessup` installer provides automated installation and update management.

```bash
# Download and run the installer
curl -sSL https://raw.githubusercontent.com/soundnesslabs/soundness-layer/main/soundnessup/install | bash

# Update shell environment
source ~/.bashrc  # For Bash
source ~/.zshenv  # For Zsh

# Install Soundness CLI
soundnessup install

# Update to latest version
soundnessup update
```

### Method 2: Docker Installation

```bash
# Build the Docker image
docker compose build

# Run CLI commands through Docker
docker compose run --rm soundness-cli [command] [options]

# Example usage
docker compose run --rm soundness-cli generate-key --name production-key
```

### Method 3: Source Installation

```bash
# Clone repository
git clone https://github.com/soundnesslabs/soundness-layer.git
cd soundness-layer/soundness-cli

# Build and install
cargo install --path .

# Verify installation
soundness-cli --version
```

## Architecture

The Soundness CLI is built with a modular architecture consisting of several key components:

### Core Modules

#### CLI Module (`cli.rs`)
Defines the command-line interface structure using the `clap` crate. Provides comprehensive help text and argument validation for all commands and subcommands.

#### Cryptography Module (`crypto.rs`)
Implements all cryptographic operations including:
- Ed25519 key pair generation and management
- AES-256-GCM encryption for key storage
- PBKDF2 key derivation with 600,000 iterations
- Digital signature generation and verification
- BIP39 mnemonic phrase handling

#### Key Store Module (`keystore.rs`)
Manages persistent storage of encrypted key pairs:
- JSON-based local key storage
- Atomic file operations for data integrity
- Key import/export functionality
- Multiple keystore support

#### Client Module (`client.rs`)
Handles network communication with the Soundness Layer:
- HTTP client implementation with proper error handling
- Automatic detection of file paths vs. blob IDs
- Request signing and authentication
- Response parsing and formatting

#### Types Module (`types.rs`)
Defines all data structures and enums used throughout the application:
- Proving system enumeration with validation
- Game type definitions
- Key pair and encryption structures
- Server response parsing

#### Utilities Module (`utils.rs`)
Provides helper functions for:
- Progress bar display during operations
- Blob ID detection and validation
- Server response formatting
- Error handling utilities

### Security Architecture

The CLI implements multiple layers of security:

1. **Key Storage**: Private keys are encrypted using AES-256-GCM with random nonces
2. **Key Derivation**: PBKDF2-HMAC-SHA256 with 600,000 iterations and 32-byte salts
3. **Authentication**: Ed25519 digital signatures for all server communications
4. **Memory Management**: Secure password caching with automatic invalidation
5. **Input Validation**: Comprehensive validation of all user inputs and file formats

## Usage

### Initial Setup

Before using the CLI, you need to generate or import a cryptographic key pair:

```bash
# Generate a new key pair
soundness-cli generate-key --name my-production-key

# Import existing key from mnemonic
soundness-cli import-key --name imported-key --mnemonic "word1 word2 ... word24"
```

### Basic Operations

#### Key Management

List all available keys:
```bash
soundness-cli list-keys
```

Display mnemonic for recovery:
```bash
soundness-cli show-mnemonic --name my-production-key
```

#### Proof Submission

Submit proof with local files:
```bash
soundness-cli send \
    --proof-file ./proofs/my-proof.bin \
    --elf-file ./programs/my-program.elf \
    --key-name my-production-key \
    --proving-system sp1
```

Submit proof using Walrus blob IDs:
```bash
soundness-cli send \
    --proof-file blob_abc123xyz789 \
    --elf-file blob_def456uvw012 \
    --key-name my-production-key \
    --proving-system risc0
```

Submit proof for predefined games:
```bash
soundness-cli send \
    --proof-file ./game-proof.bin \
    --game 8-queens \
    --key-name my-production-key \
    --proving-system ligetron \
    --payload '{"player": "alice", "difficulty": "hard"}'
```

### Advanced Usage


```

#### Working with Multiple Keystores

```bash
# List keys from specific keystore file
soundness-cli load-key-store --path ./production-keys.json list-keys

# Show mnemonic from custom keystore
soundness-cli load-key-store --path ./backup-keys.json show-mnemonic --name backup-key
```

## Command Reference

### Global Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--endpoint` | `-e` | Soundness Layer API endpoint | `https://testnet.soundness.xyz` |
| `--help` | `-h` | Show help information | - |
| `--version` | `-V` | Show version information | - |

### Key Management Commands

#### generate-key
Generate a new Ed25519 key pair with secure local storage.

```bash
soundness-cli generate-key --name <KEY_NAME>
```

**Options:**
- `--name` / `-n`: Unique identifier for the key pair (required)

**Security Notes:**
- Private key is encrypted with user-provided password
- Displays BIP39 mnemonic phrase for recovery
- Stores public key in base64 format for easy sharing

#### import-key
Import an existing key pair from a BIP39 mnemonic phrase.

```bash
soundness-cli import-key --name <KEY_NAME> --mnemonic "<MNEMONIC_PHRASE>"
```

**Options:**
- `--name` / `-n`: Name for the imported key pair (required)
- `--mnemonic` / `-m`: 24-word BIP39 mnemonic phrase (required)

#### list-keys
Display all stored key pairs with their public keys.

```bash
soundness-cli list-keys
```

**Output Format:**
```
Available key pairs:
- production-key (Public key: base64_encoded_public_key)
- test-key (Public key: base64_encoded_public_key)
```

#### show-mnemonic
Display the BIP39 mnemonic phrase for a specific key pair.

```bash
soundness-cli show-mnemonic --name <KEY_NAME>
```

**Options:**
- `--name` / `-n`: Name of the key pair (required)

**Security:**
- Requires password to decrypt the private key
- Displays warning about mnemonic security

### Proof Submission Command

#### send
Submit a zero-knowledge proof to the Soundness Layer for verification.

```bash
soundness-cli send [OPTIONS]
```

**Required Options:**
- `--proof-file` / `-p`: Path to proof file or Walrus blob ID
- `--key-name` / `-k`: Name of the signing key pair
- `--proving-system` / `-s`: ZK proving system used

**Optional Options:**
- `--elf-file` / `-l`: Path to ELF file or Walrus blob ID
- `--game` / `-g`: Predefined game type (alternative to ELF file)
- `--payload` / `-d`: Additional JSON payload

**Proving System Values:**
- `sp1`: SP1 RISC-V based ZK-VM
- `ligetron`: Ligetron high-performance prover
- `risc0`: RISC Zero general-purpose ZK-VM
- `noir`: Noir domain-specific language
- `starknet`: StarkNet Ethereum L2 solution
- `miden`: Miden VM stack-based ZK-VM

**Game Type Values:**
- `tic-tac-toe`, `tictactoe`: Tic-tac-toe game
- `8-queens`, `8queens`, `eight-queens`, `eightqueens`: Eight queens puzzle

### Keystore Management Commands

#### load-key-store
Work with keystore files in custom locations.

```bash
soundness-cli load-key-store --path <PATH> <SUBCOMMAND>
```

**Subcommands:**
- `list-keys`: List all keys in the specified keystore
- `show-mnemonic --name <KEY_NAME>`: Show mnemonic from the specified keystore

## Security

### Encryption Standards

The CLI implements industry-standard cryptographic practices:

**Key Storage:**
- Algorithm: AES-256-GCM (Authenticated Encryption)
- Key Derivation: PBKDF2-HMAC-SHA256
- Iterations: 600,000 (exceeds OWASP recommendations)
- Salt Length: 32 bytes (256 bits)
- Nonce Length: 12 bytes (96 bits, GCM standard)

**Digital Signatures:**
- Algorithm: Ed25519 (Curve25519 + SHA-512)
- Key Length: 32 bytes (256 bits)
- Signature Length: 64 bytes (512 bits)

**Mnemonic Generation:**
- Standard: BIP39
- Entropy: 256 bits (24 words)
- Language: English wordlist

### Security Best Practices

1. **Password Requirements:**
   - Use strong, unique passwords for key encryption
   - Passwords are never stored or transmitted
   - Consider using a password manager

2. **Mnemonic Storage:**
   - Write down mnemonic phrases on paper
   - Store in multiple secure locations
   - Never share or store electronically

3. **Key Management:**
   - Generate keys on secure, offline systems when possible
   - Regularly backup keystore files
   - Use different keys for different environments (test/prod)

4. **Network Security:**
   - Always verify endpoint URLs before submission
   - Use HTTPS endpoints in production
   - Monitor for suspicious network activity

### Threat Model

The CLI is designed to protect against:
- **Local File Access**: Encrypted key storage prevents unauthorized access
- **Network Interception**: HTTPS and signature verification prevent MITM attacks
- **Key Compromise**: Mnemonic recovery allows key regeneration
- **Brute Force**: High iteration count makes password cracking impractical

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SOUNDNESS_ENDPOINT` | Default API endpoint | `https://testnet.soundness.xyz` |
| `SOUNDNESS_KEYSTORE` | Default keystore file path | `./key_store.json` |

### File Locations

**Default Keystore:**
- Location: `./key_store.json` (current directory)
- Format: JSON with encrypted key pairs
- Permissions: Readable by owner only (recommended)

**Configuration Files:**
- The CLI uses command-line arguments and environment variables
- No persistent configuration file is required

## Examples

### Complete Workflow Example

```bash
# 1. Generate a new key pair
soundness-cli generate-key --name demo-key

# 2. List available keys
soundness-cli list-keys

# 3. Submit a proof for the eight queens game
soundness-cli send \
    --proof-file ./eight-queens-proof.bin \
    --game 8-queens \
    --key-name demo-key \
    --proving-system ligetron \
    --payload '{"solution": [0,4,7,5,2,6,1,3]}'

# 4. Submit a custom program proof
soundness-cli send \
    --proof-file ./custom-proof.bin \
    --elf-file ./custom-program.elf \
    --key-name demo-key \
    --proving-system sp1
```

## Troubleshooting

### Common Issues

**Key Generation Fails:**
```
Error: Failed to generate key pair
```
- Solution: Ensure sufficient entropy available on system
- Check system permissions for file creation
- Verify Rust crypto libraries are properly installed

**Authentication Errors:**
```
Error: Invalid signature
```
- Verify key name exists: `soundness-cli list-keys`
- Check password is correct when prompted
- Ensure key hasn't been corrupted: try showing mnemonic

**Network Connection Issues:**
```
Error: Failed to send request
```
- Verify endpoint URL is correct and accessible
- Check network connectivity and firewall settings
- Ensure endpoint supports HTTPS if using secure URLs

**File Not Found Errors:**
```
Error: Failed to read proof file
```
- Verify file paths are correct and files exist
- Check file permissions are readable
- For blob IDs, ensure they are valid Walrus identifiers

### Debug Mode

Enable verbose logging by setting the RUST_LOG environment variable:

```bash
export RUST_LOG=debug
soundness-cli send [options]
```

### Recovery Procedures

**Lost Keystore File:**
1. Locate mnemonic phrase backup
2. Use `import-key` command to restore key
3. Verify public key matches expected value

**Forgotten Password:**
1. Cannot be recovered - passwords are not stored
2. Use mnemonic phrase to restore key with new password
3. Update any dependent systems with new keystore

**Corrupted Keystore:**
1. Stop using corrupted keystore immediately
2. Restore from mnemonic phrase backup
3. Verify all keys are properly restored before resuming operations

### Getting Help

For additional support:
- GitHub Issues: Report bugs and feature requests
- Documentation: -
- Community Discord: Real-time support and discussions

---

**Version:** 0.1.2  
**License:** MIT  
**Maintainer:** Soundness Layer Team