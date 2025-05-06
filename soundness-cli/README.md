# Soundness CLI🔈 Soundness Testnet Registration
🎁 Potensial Reward For All Users

➖Register
https://soundness.xyz/
➖Submit Email
➖Done

➖Generate Your Key here :
https://github.com/SoundnessLabs/soundness-layer/tree/main/soundness-cli
➖Save Phrase & Public Keys
➖Join Discord & Verify
➖Submit your key on #testnet-access
➖Done

🔖 Tutorial for Codespace Users

➖For install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf
https://sh.rustup.rs | sh

🔖 After it run this 
source $HOME/.cargo/env

🔔 Source :
https://x.com/SoundnessLabs/status/1902389758527152586

A command-line interface tool for interacting with Soundness Layer testnet.

## Quick Installation

Install the CLI with a single command:

```bash
curl -sSL https://raw.githubusercontent.com/soundnesslabs/soundness-layer/main/soundnessup/install | bash
```

After installation, restart your terminal or run:
```bash
source ~/.bashrc  # for bash
# or
source ~/.zshenv  # for zsh
```

Then you can use the CLI:
```bash
soundnessup install  # Install the CLI
soundnessup update   # Update to the latest version
```

## Manual Installation

If you prefer to install manually, you can use Cargo:

```bash
cargo install --path .
```

## Testnet Registration
First of all, please follow us on [X](https://x.com/SoundnessLabs).
To join the testnet, generate your keys and head to our `testnet-access` channel in [Discord](https://discord.gg/F4cGbdqgw8) and request access using:

```bash
!access <base64-encoded-public-key>
```

## Usage

### Generating a Key Pair

To generate a new key pair for signing requests:

```bash
soundness-cli generate-key --name my-key
```

### Importing a Key Pair

To import an existing key pair from a mnemonic phrase:

```bash
soundness-cli import-key --name my-key
```

### Listing Key Pairs

To view all stored key pairs and their associated public keys:

```bash
soundness-cli list-keys
```

### Exporting Key Mnemonic

To export the mnemonic phrase for a stored key pair:

```bash
soundness-cli export-key --name my-key
```

> ⚠️ **Warning**: Keep your mnemonic phrase secure and never share it with anyone. Anyone with your mnemonic can access your key pair.

<!-- ### Sending Proofs

To send a proof and ELF file to the testnet server:

```bash
soundness-cli send --proof-file path/to/proof.proof --elf-file path/to/program.elf --key-name my-key
```

The request will be automatically signed using the specified key pair. -->
