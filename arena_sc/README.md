# Arena Battle Smart Contract

A MultiversX smart contract for organizing battles between soldier NFTs with entrance fees and rewards.

## Contract Methods

### createGame()
Creates a new battle in the arena:
- Requires a Soldier NFT and EGLD entrance fee
- Validates that the NFT is a Soldier
- Creates a new game with the initiator's soldier and entrance fee
- Game is open for another player to join

### joinGame(game_id)
Joins an existing battle:
- Requires a Soldier NFT and matching EGLD entrance fee
- Validates that the NFT is a Soldier
- Automatically starts the fight after joining
- Cannot join already full games

### View Methods

#### getGame(game_id)
Returns information about a specific game:
- Initiator address and soldier
- Entrance fee
- Competitor address and soldier (if joined)

#### getNextGameId()
Returns the ID that will be assigned to the next created game

## Battle Mechanics

### Power Calculation
- Total power = Defense + Attack values from soldier attributes
- Power is extracted from the NFT's attributes

### Win Chance Calculation
- Base 50/50 chance
- Power difference influences win probability
- Maximum 100% win chance with power difference ≥ 100

## Rewards System
- Winner receives:
  - Their Soldier NFT back
  - Both entrance fees (2x EGLD)
- Loser's Soldier NFT is burned

## Game Structure
Each game stores:
- Initiator's address and soldier NFT
- Entrance fee amount
- Competitor's address and soldier NFT (when joined)
- Game state managed through IDs

## Note
- Games are automatically resolved when a competitor joins
- Results are determined by power difference and random factor
- All soldier validations are performed before battle