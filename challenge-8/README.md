# Challenge #8: Claim Specific Tokens

Script that allows user to claim specific tokens

## Description

This script interacts with the smart contract deployed at the address `erd1qqqqqqqqqqqqqpgqlaa66qc2uapx5ef79a4csqu2cgqpr0ty6dkqpl73p8`. The smart contract was deployed using one of the wallets generated in Challenge #1. 

If you want to use a different smart contract address, please refer to the "snow_sc" instructions for deploying your own contract.

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
ts-node claim_user_tokens.ts