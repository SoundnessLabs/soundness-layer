# Soundness CLI

A command-line interface tool for interacting with Soundness Layer testnet.

## Installation

**Prerequisite:** Ensure you have the [Rust toolchain](https://rustup.rs/) installed.

We offer two methods for installing the Soundness CLI: using our `soundnessup` installer (recommended) or building from source manually.


### Recommended: Quick Install via `soundnessup`

The `soundnessup` tool manages your Soundness CLI installation and makes updates easy.

**1. Run the installer script:**
This command downloads and runs the `soundnessup` installer.

```bash
curl -sSL https://raw.githubusercontent.com/SoundnessLabs/soundness-layer/main/soundnessup/install | bash
```

**2. Update your shell environment:**
After installation, you need to update your current shell's PATH to recognize the `soundnessup` command. Either restart your terminal or run one of the following commands:
```bash
# For Bash:
source ~/.bashrc

# For Zsh:
source ~/.zshenv
```

**3. Install the CLI:**
Now, use `soundnessup` to install the Soundness CLI:

```bash
soundnessup install
```

You can later update the CLI to the latest version by running:

```bash
soundnessup update
```

### Docker Installation

You can also build and run the CLI using Docker:

```bash
# Build the Docker image
docker compose build

# Run the CLI (replace [command] with any soundness-cli command)
docker compose run --rm soundness-cli [command]

# Example: Generate a new key pair
docker compose run --rm soundness-cli generate-key --name your-key-name
```

### Manual Installation (from Source)

If you prefer to install from source, you can use Cargo.

**Build and install:**
Navigate to the `soundness-cli` directory and run:

```bash
cargo install --path .
```

## Testnet Instructions

Welcome to the Soundness Layer Testnet! Follow these steps to get started with playing ZK games and verifying proofs on-chain.

### Step 1: Get Access

To join the testnet, you'll need either the `Onboarded` role from our Discord or a special invite code.

1.  **Join our Discord:** Hop into the [Soundness Labs Discord](https://discord.gg/SoundnessLabs) and get the `Onboarded` role to participate.
2.  **Follow us on X:** Keep an eye on our [X account](https://x.com/SoundnessLabs). We regularly post invite codes for our community.

### Step 2: Prepare Your Key

Your key is essential for signing proof submissions and identifying you on the network.

**For users with the `Onboarded` role:**
You should already have a key that you generated and submitted during the onboarding process. Make sure you have it ready. You do not need to generate a new one.

**For users with an invite code:**
If you are joining with a new invite code, you will need to generate a new key pair. Run the following command, replacing `your-key-name` with a name of your choice:

```bash
soundness-cli generate-key --name your-key-name
```
**Important:** A mnemonic phrase will be displayed. **Save it in a safe place!** This is the only way to recover your key if it's lost.

### Step 3: Play a Game and Send Your Proof

Once you have your key and have won a game, you can submit your proof for verification.

Use the `send` command with the following format:

```bash
soundness-cli send --proof-file <proof-blob-id> --game <game-name> --key-name <your-key-name> --proving-system ligetron --payload '<json-payload>'
```

**Command Breakdown:**

* `--proof-file` (`-p`): The unique Walrus Blob ID for your proof, which you receive after winning a game.
* `--game` (`-g`): The name of the game you played (e.g., `8queens` or `tictactoe`).
* `--key-name` (`-k`): The name you chose for your key in Step 2.
* `--proving-system` (`-s`): The ZK proving system. For our current testnet games, this is `ligetron`.
* `--payload` (`-d`): A JSON string with the specific inputs required to verify your Ligetron proof.

Get ready to play, prove, and verify on the Soundness Layer!

## Usage

### Generating a Key Pair

To generate a new key pair for signing requests:

```bash
soundness-cli generate-key --name my-key
```

This will:

1. Generate a new Ed25519 key pair
2. Store the key pair securely in a local `key_store.json` file
3. Display the public key in base64 format

The public key will be displayed in the format:

```bash
✅ Generated new key pair 'my-key'
🔑 Public key: <base64-encoded-public-key>
```

### Importing a Key Pair

If you saved your mnemonic previously, you can import it to `key_store.json` by using following command:

```bash
soundness-cli import-key --name <name> --mnemonic "<mnemonic>"
```

If it was successful you'll get:

```bash
✅ Imported key pair '<imported-key-name>'
🔑 Public key: <base64-encoded-public-key>
```

### Listing Key Pairs

To view all stored key pairs:

```bash
soundness-cli list-keys
```

This will display all available key pairs and their associated public keys.

### Sending Proofs

The CLI supports two ways to send proofs to the server:

#### 1. Using Local Files

To send a proof and ELF Program file using local file paths:

```bash
soundness-cli send --proof-file path/to/proof.proof --elf-file path/to/program.elf --key-name my-key
```

#### 2. Using Files Stored as Walrus Blob IDs

To send a proof and ELF Program file using Walrus Blob IDs (when files are already stored in Walrus):

```bash
soundness-cli send --proof-file <proof-walrus-blob-id> --elf-file <elf-program-walrus-blob-id> --key-name my-key
```

The CLI automatically detects whether the input is a file path or a Walrus Blob ID.

#### Mixed Usage

You can also mix file paths and Walrus Blob IDs:

```bash
# Proof from file, ELF from Walrus storage
soundness-cli send --proof-file path/to/proof.proof --elf-file <walrus-blob-id> --key-name my-key

# Proof from Walrus storage, ELF from file  
soundness-cli send --proof-file <walrus-blob-id> --elf-file path/to/program.elf --key-name my-key
```

#### Proving Systems

You can specify the proving system to use:

```bash
soundness-cli send --proof-file <path-or-blob-id> --elf-file <path-or-blob-id> --key-name my-key --proving-system <sp1||ligetron||risc0>
```

Supported proving systems: `sp1`, `ligetron`, `risc0`.

The request will be signed using the specified key pair.
