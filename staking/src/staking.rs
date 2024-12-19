#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait Staking {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(stake_token_winter)]
    fn stake_token_winter(&self, payment: EgldOrEsdtTokenPayment) {
        let caller = self.blockchain().get_caller();
        require!(payment.amount > 0, "Must pay more than 0");

        self.locked_token_amount(&caller).update(|current_amount| current_amount.clone() + &payment.amount);
        self.locked_tokens().insert(caller);
    }

    #[view(lockedTokenAmount)]
    #[storage_mapper("lockedTokenAmount")]
    fn locked_token_amount(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(lockedTokens)]
    #[storage_mapper("lockedTokens")]
    fn locked_tokens(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(stakeEpoch)]
    #[storage_mapper("stakeEpoch")]
    fn stake_epoch(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;
}
