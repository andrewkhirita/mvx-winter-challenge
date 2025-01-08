# Challenge #14: Generate Resource

Script that allows users (wallets generated at challenge #1) to generate resources (WOOD, FOOD, STONE, GOLD) after staking their WINTER tokens.

## Description

This script interacts with the smart contracts deployed at the address 
`erd1qqqqqqqqqqqqqpgqrqds9gxytvqxmtu09qvjkw4r97z497de6dkqv2xnkx` -> WOOD_SC
`erd1qqqqqqqqqqqqqpgqrad0al0gqpnqg68e9t4zfg853urg97au6dkqjz6mh3` -> FOOD_SC
`erd1qqqqqqqqqqqqqpgq23y2x76aawjjwtfdmuuqsecshvwq459m6dkqy5jyqh` -> STONE_SC
`erd1qqqqqqqqqqqqqpgq5uujszsw0n2tvqccedjxvtl6lsau7atu6dkqhnu7jn` -> GOLD_SC

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
ts-node generate_resources.ts