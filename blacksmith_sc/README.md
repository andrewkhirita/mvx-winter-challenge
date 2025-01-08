# Blacksmith Tool Crafting Smart Contract

A MultiversX smart contract for crafting tools (shields and swords) using various resources.

## Contract Methods

### issue(name, ticker)
Issues the TOOL NFT collection:
- Requires EGLD payment for issuance
- Sets up the NFT collection with all roles
- Stores the token identifier for future minting

### Shield Crafting

#### mintShield()
Initiates the shield crafting process:
- Requires 2 ORE tokens
- Burns the required resources
- Sets a claim timer for 600 blocks (approximately 1 hour)
- Caller must wait for timer before claiming

#### claimShield()
Claims the crafted shield:
- Can only be called after the waiting period
- Mints a SHIELD NFT
- Sends the NFT to the caller
- Clears the pending claim

### Sword Crafting

#### mintSword()
Initiates the sword crafting process:
- Requires 1 GOLD token and 3 ORE tokens
- Burns the required resources
- Sets a claim timer for 600 blocks (approximately 1 hour)
- Caller must wait for timer before claiming

#### claimSword()
Claims the crafted sword:
- Can only be called after the waiting period
- Mints a SWORD NFT
- Sends the NFT to the caller
- Clears the pending claim

## Resource Requirements

### Shield
- ORE: 2 tokens

### Sword
- GOLD: 1 token
- ORE: 3 tokens

## Timing
- All crafting operations require a waiting period of 600 blocks (~ 1 hour)
- Items can be claimed only after the waiting period has passed