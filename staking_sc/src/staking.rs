#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const ISSUE_COST: u64 = 50_000_000_000_000_000; // 0.05 EGLD

#[multiversx_sc::contract]
pub trait Staking {
    #[upgrade]
    fn upgrade(&self) {}
    
    #[init]
    fn init(&self) {
    }

    #[payable("*")]
    #[endpoint(stakeTokenWinter)]
    fn stake_token_winter(&self) {
        let payment = self.call_value().single_esdt();
        
        let winter_ticker = ManagedBuffer::from(b"WINTER");
        require!(
            payment.token_identifier.ticker().eq(&winter_ticker),
            "Only WINTER tokens can be staked"
        );

        let caller = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();
        let current_timestamp = self.blockchain().get_block_timestamp();

        if !self.staking_position(&caller).is_empty() {
            self.staking_position(&caller).update(|position| {
                position.stake_amount += payment.amount;
            });
        } else {
            let new_position = StakingPosition {
                stake_amount: payment.amount,
                last_claim_timestamp: current_timestamp,
                stake_epoch: current_epoch,
                reward_recipient: None,
            };
            self.staking_position(&caller).set(new_position);
        }
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();
        let position = self.staking_position(&caller).get();
        
        require!(position.stake_amount > 0, "Nothing staked");
        
        require!(
            self.blockchain().get_block_epoch() >= position.stake_epoch + 5,
            "Staking period not met"
        );
        require!(
            self.blockchain().get_block_timestamp() >= position.last_claim_timestamp + 86400,
            "Can claim once per 24h"
        );
    
        let reward = &position.stake_amount / 100u64;
        
        if reward > 0 {
            let recipient = position.reward_recipient.unwrap_or_else(|| caller.clone());
            let reward_token = self.token_manager().get_token_id();
            
            self.send().esdt_local_mint(
                &reward_token,
                0,
                &reward
            );
            
            self.send().direct_esdt(
                &recipient,
                &reward_token,
                0,
                &reward
            );
    
            self.staking_position(&caller).update(|pos| {
                pos.last_claim_timestamp = self.blockchain().get_block_timestamp();
            });
        }
    }

    #[endpoint(setRewardRecipient)]
    fn set_reward_recipient(&self, recipient: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        self.staking_position(&caller).update(|position| {
            position.reward_recipient = Some(recipient);
        });
    }

    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(&self, name: ManagedBuffer, ticker: ManagedBuffer) {
        self.blockchain().check_caller_is_owner();
        let manager = self.token_manager();
        require!(manager.is_empty(), "Token already issued!");
        let amount = self.call_value().egld_value();
        require!(
            amount.clone_value() == BigUint::from(ISSUE_COST),
            "Insufficient funds!");
        self.token_manager().issue_and_set_all_roles(amount.clone_value(), name, ticker, 0, Some(self.callbacks().issue_callback()));
    }


    #[callback]
    fn issue_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                    self.token_manager().set_token_id(token_id);
            }
            ManagedAsyncCallResult::Err(_) => { }
        }
    }

    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, addr: &ManagedAddress) -> SingleValueMapper<StakingPosition<Self::Api>>;

    #[storage_mapper("rewardTokenId")]
    fn reward_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("ticker")]
    fn token_manager(&self) -> FungibleTokenMapper<Self::Api>;
}

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug)]
pub struct StakingPosition<M: ManagedTypeApi> {
    pub stake_amount: BigUint<M>,
    pub last_claim_timestamp: u64,
    pub stake_epoch: u64,
    pub reward_recipient: Option<ManagedAddress<M>>,
}
