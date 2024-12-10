# MultiversX Wallet Generator

Simple script to generate MultiversX wallets across different shards on devnet.

## Prerequisites

- Node.js
- TypeScript
- MultiversX SDK

## Setup

```bash
npx tsc --init
npm install @multiversx/sdk-core @multiversx/sdk-network-providers @multiversx/sdk-wallet
```

## Usage

Run the script:
```bash
ts-node generate_wallets.ts
```

This will:
1. Generate 9 unique wallets (3 per shard)
2. Save them in the `wallets` folder as JSON files

## Token Request Verification

Tokens were manually requested using the [MultiversX Web Wallet Faucet](https://docs.multiversx.com/wallet/web-wallet/#testnet-and-devnet-faucet).

Transaction hashes:
```
Shard 0:
- a18f1104a88f35138c432031841830dc7e59f78e1e8d398d532bb7cf5e7c69ee
- 25dacb109cfc902fe867810a3aea1a3c7e9a67e8d5171fdbb3fb09758a5edb9d
- a87c9e55bc1a6f52c975e4bb3d4ed04d7bafcc45434a470b7b5516792a2f18d8

Shard 1:
- 850393d9e558ab03318a8f8f9e5ef2c1652e7533e74df386021fd3f1e43790fe
- ad5f8ee2329dffd069e000dd088e8738ac2585901e83379bf5515999a3ce500a
- 97bce944af2778d1b2a5f6feff22c19bad26f1bd31cc5c1e76ba6de14ecab21c

Shard 2:
- b2754e8b8ec3ee765d1f0ebe2b159d54ca848fd676a4d2bf2ac8a79815bebebf
- 8137fe35cb52c6afda29e3a14af4563651f3a8353475ec9423b0df68e18d24b2
- a7824ecbcc26173be196d06f7b00486637987b2ed2b8f899853edf35afbce4ba
```