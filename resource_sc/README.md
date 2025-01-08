# Resource Generation Smart Contract

A MultiversX smart contract for generating game resources by staking WINTER tokens.

## Contract Methods

### stakeWinter()
Allows users to stake WINTER tokens:
- Requires minimum 1000 WINTER tokens
- Records stake amount and initial claim round
- Higher stakes generate resources faster

### generateResources(resource_type)
Generates resources based on staked amount:
- Calculates resources based on stake multiplier (stake/1000)
- Different resources have different generation rates
- Stores pending resources for claiming
- Updates last claim round

### claimResources(resource_type)
Claims generated resources:
- Mints the pending resources
- Sends resources to the caller
- Clears pending resources after claiming

### issueToken(resource_type, name, ticker)
Issues new resource tokens:
- Can create FOOD, WOOD, STONE, or GOLD tokens
- Each resource type can only be issued once
- Sets up resource configuration with generation rates

## Resource Types and Generation Rates

### Resource Configurations
- WOOD: Generates every 600 round
- FOOD: Generates every 1200 round
- STONE: Generates every 1800 round
- GOLD: Generates every 2400 rounds

## Staking Mechanics
- Minimum stake: 1000 WINTER tokens
- Generation multiplier: Stake amount / 1000
- Example: 2000 WINTER staked = 2x resource generation

## Storage
The contract maintains:
- Resource configurations for each type
- Stake amounts per address
- Last claim round per address
- Pending resources per address and type