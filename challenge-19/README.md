# Challenge #19: Mint SHIELD NFT

Script that allows users (wallets generated at challenge #1) to mint SHIELD NFT
## Description

This script interacts with the smart contract deployed at the address `erd1qqqqqqqqqqqqqpgq9hvgwd5dpa8md9hw73uf557z8xjuwua46dkqunflzv`

The smart contract was deployed using one of the wallets generated in Challenge #1. 

If you want to use a different smart contract address, please refer to the "Deploying-Contract.md" instructions for deploying your own contract.

## Prerequisites

- Node.js
- TypeScript
- MultiversX SDK
- Wallets generated from Challenge #1

## Setup

```bash
npx tsc --init
npm install @multiversx/sdk-core @multiversx/sdk-network-providers @multiversx/sdk-wallet
```

## Usage

Run the script:
```bash
ts-node mint_shield.ts
