# Challenge #1: Token Issuer

Issue tokens for each wallet generated in challenge #1

## Prerequisites

- Node.js
- TypeScript
- MultiversX SDK
- Wallets generated from challenge #1


## Setup

```bash
npx tsc --init
npm install @multiversx/sdk-core @multiversx/sdk-network-providers @multiversx/sdk-wallet
```

## Usage

Run the script:
```bash
ts-node issue_tokens.ts
```

This will:
1. Generate 9 unique wallets (3 per shard)
2. Save them in the `wallets` folder as JSON files