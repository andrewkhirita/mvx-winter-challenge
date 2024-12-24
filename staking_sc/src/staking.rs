#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait Staking{

    #[upgrade]
    fn upgrade(&self) {}

    #[init]
    fn init(&self) {
        // Set default rewards recipient as contract owner
        self.rewards_recipient().set(&self.blockchain().get_caller());
    }

    #[payable("*")]
    #[endpoint(stakeTokenWinter)]
    fn stake_token_winter(&self) {
        for payment in self.call_value().all_esdt_transfers().iter() {
            require!(
                payment.token_identifier.as_managed_buffer().to_boxed_bytes().as_ref().starts_with(b"WINTER-"),
                "Only WINTER tokens can be staked"
            );
            require!(payment.amount > 0, "Must stake more than 0");
        
            let caller = self.blockchain().get_caller();
            let current_epoch = self.blockchain().get_block_epoch();
            
            self.stake_epoch(&caller).set(&current_epoch);
            self.last_reward_claim(&caller).set(&self.blockchain().get_block_timestamp());
            self.staked_amount(&caller).update(|amount| *amount += payment.amount);
        }        
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();
        require!(self.staked_amount(&caller).get() > 0, "Nothing staked");
        
        let current_epoch = self.blockchain().get_block_epoch();
        let stake_epoch = self.stake_epoch(&caller).get();
        require!(current_epoch >= stake_epoch + 5, "Staking period not met");

        let current_timestamp = self.blockchain().get_block_timestamp();
        let last_claim = self.last_reward_claim(&caller).get();
        
        // Check if 24h (86400 seconds) have passed
        require!(current_timestamp >= last_claim + 86400, "Can claim once per 24h");

        // Calculate 1% of staked amount
        let reward = &self.staked_amount(&caller).get() / 100u64;
        
        if reward > 0 {
            let reward_recipient = self.rewards_recipient().get();
            let reward_token = TokenIdentifier::from("SNOW-ab6b96".as_bytes());
            self.send().direct_esdt(&reward_recipient, &reward_token, 0, &reward);
            self.last_reward_claim(&caller).set(&current_timestamp);
        }
    }

    #[only_owner]
    #[endpoint(changeRewardsRecipient)]
    fn change_rewards_recipient(&self, new_recipient: ManagedAddress) {
        require!(!new_recipient.is_zero(), "Invalid address");
        self.rewards_recipient().set(&new_recipient);
    }

    #[storage_mapper("stakeEpoch")]
    fn stake_epoch(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("stakedAmount")]
    fn staked_amount(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("lastRewardClaim")]
    fn last_reward_claim(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("rewardsRecipient")]
    fn rewards_recipient(&self) -> SingleValueMapper<ManagedAddress>;
}