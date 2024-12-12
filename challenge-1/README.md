# Challenge #1: Wallet Generator

Script that generate MultiversX Wallets across different Shards

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
- d793138d6739c5a64052c17f6d6ea7ca0c0cd62f386e487c04b364eb7c1294b3
- c675255e67bad0547226f4048403b1b3c5d36c0c154a1738eb8fa9a9dfe0b560
- 87572190a08272c0783982192d567451de81a15b9b0bbcd1d41daa8bf68949c6

Shard 1:
- 68350301aa61458c4295a65becb880dea3be1b523a38d534f4f3deb70f9d5ada
- f849773c04609b28e90c1084e42edec9456abbdf1f96435ba6cf37467f1f9cd4
- 57b0e4c2c3e0c1774117fc387e6cdb87b538cf9024b5951598703f554f6bc2e0

Shard 2:
- 0af0ceec374753bdeac0327a4296ed2b0fb7253431ccd354b1894ad4dbd6d62a
- b8c72d5d4a086fff724fdc314a8cb5022a21083f8abca4ff09cf983223e3a77c
- 88ccadf67ce8e3cbeeaf9b4204482ab2b6722ef63f395b19b1a6bf5bc720cc39
```