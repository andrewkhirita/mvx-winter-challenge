# Citizen NFT Smart Contract

A MultiversX smart contract for managing Citizen NFTs with upgrade mechanics and equipment system.

## Contract Methods

### issue(name, ticker)
Issues the Citizen NFT collection:
- Requires EGLD payment for issuance
- Sets up the NFT collection with all roles
- Stores the token identifier for future minting

### mintCitizen()
Starts the minting process for a Citizen NFT:
- Requires 10 WOOD and 15 FOOD tokens
- Burns the required resources
- Sets a claim timer for 600 blocks (approximately 1 hour)
- Caller must wait for timer before claiming

### claimCitizen()
Claims the minted Citizen NFT:
- Can only be called after the waiting period
- Mints and sends the NFT to the caller
- Clears the pending claim

### upgradeCitizen()
Upgrades a Citizen to a Soldier:
- Requires a Citizen NFT + 5 GOLD and 5 ORE tokens
- Burns the upgrade resources
- Updates the NFT attributes to Soldier rank
- Returns the upgraded NFT to the caller

### equipShield()
Equips a shield to a Soldier:
- Requires a Soldier NFT and a TOOL token (shield)
- Increases defense attribute by 1
- Returns the updated NFT to the caller
- Can only be used on Soldiers

### equipSword()
Equips a sword to a Soldier:
- Requires a Soldier NFT and a TOOL token (sword)
- Increases attack attribute by 1
- Returns the updated NFT to the caller
- Can only be used on Soldiers

## Resource Requirements

### Minting
- WOOD: 10 tokens
- FOOD: 15 tokens

### Upgrading to Soldier
- GOLD: 5 tokens
- ORE: 5 tokens

## Character Attributes
Characters can have different ranks and stats:
- Citizen (base NFT)
- Soldier
  - Defense: 0+ (increased by shields)
  - Attack: 0+ (increased by swords)