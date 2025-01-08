# Ore Mining Smart Contract

A MultiversX smart contract for mining ORE tokens using STONE resources.

## Contract Methods

### issue(name, ticker)
Issues the ORE fungible token:
- Requires EGLD payment for issuance
- Sets up the token with all roles
- Stores the token identifier for future minting

### mintOre()
Initiates the ore mining process:
- Requires 20 STONE tokens
- Burns the required STONE tokens
- Sets a claim block (next block) for the caller
- Caller must wait one block before claiming

### claimOre()
Claims the mined ORE tokens:
- Can only be called if there's a pending claim
- Requires waiting for one block after minting
- Mints 1 ORE tokens
- Sends the minted tokens to the caller
- Clears the pending claim

## Resource Requirements

### Mining
- STONE: 20 tokens

## Rewards
- Each successful mining operation yields 1 ORE tokens

## Storage
The contract maintains:
- Ore token identifier
- Token manager for the fungible token
- Next nonce tracker
- Pending claims mapping per address