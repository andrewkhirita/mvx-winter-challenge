# Challenge #3: Transfer tokens

Script that transfers 10,000 units of each issued token (challenge #2) from each generated account (challenge #1) to 1000 other accounts using API

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
ts-node transfer_tokens.ts
```

## Important Notes
### Rate Limiting
Public MultiversX APIs have a rate limit mechanism that brings the following limitations:
* devnet-api.multiversx.com (*devnet*): 5 requests / IP / second

To handle these limitations efficiently, the script:
- Implements parallel processing for concurrent transactions
- Includes rate limiting logic to avoid API throttling
- Uses batching strategies to optimize transaction throughput