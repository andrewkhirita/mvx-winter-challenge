# Challenge #23: Battle Arena

Script that allows users (wallets generated at challenge #1) to send their CITIZEN (soldier) NFT's to battle for winning prizes. Users can create games with specific entrance fee that allows other players to participate. The winner will be determined after analyzing the defense and attack attributes on each CITIZEN participant.

## Description

This script interacts with the smart contract deployed at the address `erd1qqqqqqqqqqqqqpgqhwdtf39jdex90jvq8uw9f6e0xrs8udn46dkqj7dtg9`

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
ts-node arena.ts
