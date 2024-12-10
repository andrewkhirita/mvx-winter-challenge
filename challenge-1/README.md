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
- ac974e2be482069809cae9e511520e472a8910dd019e7c0706dd021b85cb58f5
- c2918aef680222ad39ede6283b9de71ebfae446f6e70e6d83497d15ce7b1574a
- bf8db66d11fb9f7b4c807780c2ccb221a9897fc0ad0095147772f0df07dfcaba

Shard 1:
- 93e2f3832b5eddcf6df212be946606eccea8398b072e8c7ab306ba0c33e70d8e
- db3013d97bf506433e6771f9fec1c06e210735c8b25a4844c2c2b7fe80c013f5
- 55ead6be6ba0fe1e421aa34fec71dde424fc1251f187df89a2544bafd99edd8f

Shard 2:
- 5c6f597ede7bdf0cc65ef761f5b2e13b60be8be26f234c72dad622b888dd8141
- 78df54c1c5e14a971a4c36b9202f5387233dcffec7ac087a12516ffc73791d2b
- 99090541596356e32dc9a908c9c0ef6762483e4f9edc776510fb38e8a5534d5b
```