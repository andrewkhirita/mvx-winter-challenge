#![no_std]
use multiversx_sc::imports::*;

const BLOCKS_IN_DAY: u64 = 60 * 60 * 24 / 6;
const REWARD_PERCENTAGE: u64 = 100; // 1% reward rate

#[multiversx_sc::contract]
pub trait Staking {
    ////////////////////////////////////////////////////////////
    // Challenge 1 - Basic Staking Functionality
    // Requirements:
    // - Only one endpoint (stake_token_winter)
    // - Lock and hold tokens
    // - 5 epochs minimum deposit
    // - No release mechanism
    ////////////////////////////////////////////////////////////
    #[init]
    fn init(&self) {
        // Set minimum staking period (5 epochs)
        self.min_stake_epochs().set(&5u64);

        // Challenge 2 initialization
        let owner = self.blockchain().get_owner_address();
        self.rewards_recipient().set(&owner);
        self.issue_snow_token();
    }

    #[payable("*")]
    #[endpoint(stakeTokenWinter)]
    fn stake_token_winter(&self, payment: EgldOrEsdtTokenPayment) {
        let caller = self.blockchain().get_caller();
        require!(payment.amount > 0, "Must pay more than 0");

        // Store staking information
        let current_epoch = self.blockchain().get_block_epoch();
        let current_block = self.blockchain().get_block_nonce();
        
        // Challenge 2 - Process rewards if existing stake
        if self.locked_tokens().contains(&caller) {
            self.process_rewards(&caller);
        }
        
        // Basic staking functionality
        self.stake_epoch(&caller).set(&current_epoch);
        self.last_reward_block(&caller).set(&current_block);
        self.locked_token_amount(&caller).update(|current_amount| {
            *current_amount += payment.amount
        });
        self.locked_tokens().insert(caller);
    }

    ////////////////////////////////////////////////////////////
    // Challenge 2 - Reward Token Implementation
    // Requirements:
    // - Mint 1% rewards daily
    // - SNOW token with 8 decimals
    // - Automatic rewards every 24h
    ////////////////////////////////////////////////////////////
    fn issue_snow_token(&self) {
        let token_properties = FungibleTokenProperties {
            num_decimals: 8,  // Required 8 decimals
            can_freeze: true,
            can_wipe: true,
            can_pause: true,
            can_change_owner: false,
            can_upgrade: false,
            can_add_special_roles: false,
            can_burn: true,
            can_mint: true,
        };

        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                BigUint::from(50000000u64),
                &ManagedBuffer::from(b"SNOW"),
                &ManagedBuffer::from(b"SNOW"),
                &BigUint::zero(),
                token_properties
            )
            .async_call()
            .with_callback(self.callbacks().snow_token_issue_callback())
            .call_and_exit();
    }

    #[callback]
    fn snow_token_issue_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.reward_token_id().set(&token_id);
                
                self.send()
                    .esdt_system_sc_proxy()
                    .set_special_roles(
                        &self.blockchain().get_sc_address(),
                        &token_id,
                        (&[EsdtLocalRole::Mint][..]).into_iter().cloned(),
                    )
                    .async_call()
                    .call_and_exit();
            },
            ManagedAsyncCallResult::Err(_) => {
                sc_panic!("Failed to issue SNOW token");
            },
        }
    }

    // Internal function to handle automatic rewards
    fn process_rewards(&self, user: &ManagedAddress) {
        let current_block = self.blockchain().get_block_nonce();
        let last_reward = self.last_reward_block(user).get();
        
        if current_block >= last_reward + BLOCKS_IN_DAY {
            let staked_amount = self.locked_token_amount(user).get();
            let reward_amount = self.calculate_rewards(&staked_amount);
            
            if reward_amount > 0 {
                self.send()
                    .esdt_local_mint(&self.reward_token_id().get(), 0, &reward_amount);
                
                let recipient = self.rewards_recipient().get();
                self.send()
                    .direct_esdt(&recipient, &self.reward_token_id().get(), 0, &reward_amount);
                
                self.last_reward_block(user).set(&current_block);
            }
        }
    }

    fn calculate_rewards(&self, staked_amount: &BigUint) -> BigUint {
        staked_amount * REWARD_PERCENTAGE / 10_000u64 // 1% reward
    }

    ////////////////////////////////////////////////////////////
    // Challenge 3 - Rewards Recipient Management
    // Requirements:
    // - Allow changing rewards recipient
    ////////////////////////////////////////////////////////////
    #[endpoint(changeRewardsRecipient)]
    fn change_rewards_recipient(&self, new_recipient: ManagedAddress) {
        require!(!new_recipient.is_zero(), "Invalid address");
        self.rewards_recipient().set(&new_recipient);
    }


    #[storage_mapper("minStakeEpochs")]
    fn min_stake_epochs(&self) -> SingleValueMapper<u64>;

    #[view(lockedTokenAmount)]
    #[storage_mapper("lockedTokenAmount")]
    fn locked_token_amount(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(lockedTokens)]
    #[storage_mapper("lockedTokens")]
    fn locked_tokens(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("stakeEpoch")]
    fn stake_epoch(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("lastRewardBlock")]
    fn last_reward_block(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getRewardTokenId)]
    #[storage_mapper("rewardTokenId")]
    fn reward_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getRewardsRecipient)]
    #[storage_mapper("rewardsRecipient")]
    fn rewards_recipient(&self) -> SingleValueMapper<ManagedAddress>;
}