#![no_std]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait Staking {
    #[init]
    fn init(&self) {
        self.min_stake_epochs().set(&5u64);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(stakeTokenWinter)]
    fn stake_token_winter(&self, payment: EgldOrEsdtTokenPayment) {
        let caller = self.blockchain().get_caller();
        require!(payment.amount > 0, "Payment should be bigger than 0!");

        let current_epoch = self.blockchain().get_block_epoch();
        self.stake_epoch(&caller).set(&current_epoch);
        self.locked_token_amount(&caller).update(|current_amount| {
            *current_amount += payment.amount
        });

        self.locked_tokens().insert(caller);
    }

    // View function to check minimum staking period
    #[view(getMinStakeEpochs)]
    fn get_min_stake_epochs(&self) -> u64 {
        self.min_stake_epochs().get()
    }

    // View function to check stake duration for an address
    #[view(getStakeDuration)]
    fn get_stake_duration(&self, address: &ManagedAddress) -> OptionalValue<u64> {
        if !self.locked_tokens().contains(address) {
            return OptionalValue::None;
        }

        let stake_start = self.stake_epoch(address).get();
        let current_epoch = self.blockchain().get_block_epoch();
        OptionalValue::Some(current_epoch - stake_start)
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
}