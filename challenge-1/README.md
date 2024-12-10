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
- c62f81c643cc24410b25928c5d4206e685f3272561f8565ef700e4dc958cbbcf
- 0b610056d7125ec219e9ddd38307234119648d87c871a2133a739b42d0324e59
- 3d3325d1b7d5ce7dbea6586e1d58f5f0754a255fe7d4029516a7cbff999eb410

Shard 1:
- b6738082454b09b50970cc2f1295d9ed3c5eaa4336ffbf1f822c3c2d26df7a44
- fed05ba9780555193ee48474e1d4e29d12f099471e114d794407f60ee90a49d1
- d59b7edbfd6cfc54a81d7691ed5c113cedb4883a3e9c69c451c55361d868ba0e

Shard 2:
- 205797a213533164d8e01e926e95203fde4e5991078273a7ff56d2d23eb7462a
- 22cf6b8487330a27471401fb1e029ec3eb4c00efb2f8f17b1dc3315e5d380482
- d08d7052387b39873c0729b1eda070aa9cba4339b95aed91c419736e9dce47e0
```