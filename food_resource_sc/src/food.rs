#![no_std]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait Food {
    #[init]
    fn init(&self) {
        self.token_id().set(&TokenIdentifier::from("FOOD"));
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(stakeWinter)]
    fn stake_winter(&self) {
        let payment = self.call_value().single_esdt();
        let caller = self.blockchain().get_caller();
        
        require!(payment.amount >= 1000u64, "Minimum 1000 WINTER required");
        self.stake_amount(&caller).set(&payment.amount);
        self.last_claim_round(&caller).set(&self.blockchain().get_block_round());
    }

    #[endpoint(generateResources)]
    fn generate_resources(&self) {
        let caller = self.blockchain().get_caller();
        let stake = self.stake_amount(&caller).get();
        let current_round = self.blockchain().get_block_round();
        let last_claim = self.last_claim_round(&caller).get();
        
        require!(stake >= 1000u64, "No stake found");
        
        // Pentru WOOD, folosim 600 runde
        const ROUNDS_REQUIRED: u64 = 1200;
        
        require!(current_round >= last_claim + ROUNDS_REQUIRED, "Not enough rounds passed");
        
        let multiplier = &stake / 1000u64;
        let rounds_passed = (current_round - last_claim) / ROUNDS_REQUIRED;
        let amount_to_generate = multiplier * rounds_passed;
        
        if amount_to_generate > 0 {
            self.pending_resources(&caller).set(&amount_to_generate);
            self.last_claim_round(&caller).set(&current_round);
        }
    }

    #[endpoint(claimResources)]
    fn claim_resources(&self) {
        let caller = self.blockchain().get_caller();
        let amount = self.pending_resources(&caller).get();
        require!(amount > 0, "No resources to claim");
        
        self.send()
            .esdt_local_mint(&self.token_id().get(), 0, &amount);
            
        self.send()
            .direct_esdt(
                &caller,
                &self.token_id().get(),
                0,
                &amount
            );
            
        self.pending_resources(&caller).clear();
    }

    // Storage
    #[view(getTokenId)]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("stakeAmount")]
    fn stake_amount(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("lastClaimRound")]
    fn last_claim_round(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("pendingResources")]
    fn pending_resources(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;
}