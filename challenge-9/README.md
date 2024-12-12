# Challenge #9: Claim Tokens

Script that allows users (wallets generated at challenge #1) to claims the already minted tokens

## Description

This script interacts with the smart contract deployed at the address `erd1qqqqqqqqqqqqqpgqexvchcft04n883346yphv7mpfwy6klgg6dkqdsvezp`. The smart contract was deployed using one of the wallets generated in Challenge #1. 

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
ts-node claim_tokens.ts