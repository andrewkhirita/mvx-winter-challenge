# WINTER Token Staking Smart Contract

A MultiversX smart contract for staking WINTER tokens.

## Contract Methods

### stakeTokenWinter()
Allows users to stake their WINTER tokens. The method:
- Accepts only WINTER tokens as payment
- Adds the staked amount to user's existing position if they already have one
- Creates a new staking position if user is staking for the first time
- Records stake amount, timestamp, and epoch

### claimRewards()
Enables users to claim their staking rewards. The method:
- Calculates reward as 1% of staked amount
- Requires minimum 5 epochs since staking
- Can only be called once every 24 hours
- Mints and sends reward tokens to the recipient
- Updates the last claim timestamp

### setRewardRecipient(recipient)
Allows stakers to set a different address to receive their rewards:
- Updates the staking position with new recipient address
- All future rewards will be sent to this address

### issueToken(name, ticker)
Restricted to contract owner only:
- Issues the reward token for the contract
- Requires 0.05 EGLD payment for token issuance
- Can only be called once (token can't be reissued)
- Sets all required token roles for the contract

### Storage
The contract maintains:
- Staking positions for each address
- Reward token identifier
- Token manager for handling the reward token