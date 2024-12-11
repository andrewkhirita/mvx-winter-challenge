# Challenge #5: Issue Tokens Using Smart Contract

Script that interacts with a smart contract and allows users to issue tokens by calling an endpoint.

## Description

This script interacts with the smart contract deployed at the address `erd1qqqqqqqqqqqqqpgquuqzmlvqz7qdgnnfl5qwt50ncxw08y70896qephqm0`. The smart contract was deployed using one of the wallets generated in Challenge #1. 

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
ts-node issue_token_by_sc.ts