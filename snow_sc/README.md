# Snow Token Management Smart Contract

A MultiversX smart contract for token management operations.

## Contract Methods

### issueTokenSnow(token_name, token_ticker, initial_supply)
Issues a new fungible token with the following properties:
- Requires EGLD payment for issuance
- Creates token with 8 decimals
- Includes all special properties: freeze, wipe, pause, change owner, upgrade, special roles, burn, mint
- Sets initial token supply
- Stores token information in user's balance and issued tokens list

### burnTokens(token_identifier, amount)
Allows burning of tokens:
- Verifies if there's sufficient balance for burning
- Updates the token balance after burning
- Performs the local burn operation

### View Methods

#### getAllUsersTokens(address)
Returns all tokens issued by a specific address:
- Lists all token identifiers associated with the address

#### getAllUserTokenBalances(address)
Returns all token balances for an address:
- Only callable by the address owner
- Returns pairs of (token_identifier, balance)

#### getSingleTokenBalance(token_id)
Returns the balance for a specific token:
- Shows current balance for the requested token

### Claim Methods

#### claimUserTokens(token_identifier)
Allows users to claim tokens:
- Verifies if user has claimable amount
- Sends the claimable tokens to the caller
- Clears the claimable amount after transfer

#### claimTokens()
Claims all available tokens for the caller:
- Checks if user has any issued tokens
- Transfers all available token balances to the caller
- Clears the balances after transfer

### Storage
The contract maintains:
- Token to claim identifier
- Claimable amounts per user
- Issued tokens per user
- Token balances
- User tokens list