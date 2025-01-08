# Deploying a Smart Contract

This guide explains how to deploy a smart contract using the MultiversX framework.

## Prerequisites

Before deploying a smart contract, ensure you have the following setup:

### Rust Programming Language

Install the Rust programming language on your system:

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Choose the default toolchain
rustup update
rustup default stable

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Verify installation
rustc --version
rustup show
```

### MultiversX Framework

- MultiversX Rust framework version **0.50.0** or higher.
- Install the `multiversx-sc-meta` tool for generating interactor templates:

```bash
cargo install multiversx-sc-meta
```

### MultiversX Python SDK

Install the MultiversX SDK CLI, which is required to run `mxpy` commands:

```bash
pipx install multiversx-sdk-cli --force
```

### Wallet PEM Files

Generate `.PEM` files from the `.JSON` files of the wallets created in **Challenge #1**. These PEM files are required for signing transactions during deployment.

## Setup

1. Create a directory for your smart contract project, or navigate to your existing project.
2. Ensure your project contains the necessary Rust smart contract code and configuration files.

## Usage

### Step 1: Build the Smart Contract

Build the smart contract to generate the WebAssembly (`.wasm`) bytecode:

```bash
mxpy contract build --path "<path_to_your_project>"
```

Example:
```bash
mxpy contract build --path "~/projects/mvx-smart-contracts/my_smart_contract"
```

### Step 2: Deploy the Smart Contract

Deploy the smart contract to the MultiversX blockchain:

```bash
mxpy --verbose contract deploy \
  --bytecode=<path_to_wasm_file> \
  --recall-nonce \
  --gas-limit=60000000 \
  --send \
  --pem=<path_to_pem_file> \
  --proxy=https://devnet-api.multiversx.com \
  --chain="D" \
  --metadata-payable-by-sc \
  --metadata-payable
```

Example:
```bash
mxpy --verbose contract deploy \
  --bytecode=~/projects/mvx-smart-contracts/my_smart_contract/output/contract.wasm \
  --recall-nonce \
  --gas-limit=60000000 \
  --send \
  --pem=~/wallets/my_wallet.pem \
  --proxy=https://devnet-api.multiversx.com \
  --chain="D" \
  --metadata-payable-by-sc \
  --metadata-payable
```

## Notes

- Replace `<path_to_your_project>`, `<path_to_wasm_file>`, and `<path_to_pem_file>` with the appropriate paths on your system.
- Use the `--proxy` flag to specify the network you are deploying to. For example, use `https://devnet-api.multiversx.com` for the Devnet.
- Ensure you have sufficient funds in the wallet associated with the PEM file to cover gas fees.
