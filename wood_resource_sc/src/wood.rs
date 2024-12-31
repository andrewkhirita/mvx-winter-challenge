#![no_std]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait Wood {
    #[init]
    fn init(&self) {
        self.token_id().set(&TokenIdentifier::from("WOOD"));
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
        
        //600
        const ROUNDS_REQUIRED: u64 = 20;
        
        // require!(current_round >= last_claim + ROUNDS_REQUIRED, "Not enough rounds passed");
        
        let multiplier = &stake / 1000u64;
        // let rounds_passed = (current_round - last_claim) / ROUNDS_REQUIRED;
        let amount_to_generate = multiplier * ROUNDS_REQUIRED;
        
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

    #[callback]
    fn wood_token_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.token_id().set(&token_id);
            },
            ManagedAsyncCallResult::Err(_) => sc_panic!("Token issue failed")
        }
    }

    #[payable("EGLD")]
    #[endpoint(issue)]
    fn issue_token(&self, name: ManagedBuffer, ticker: ManagedBuffer) {
        let amount = self.call_value().egld_value();
        self.token_manager().issue_and_set_all_roles(amount.clone_value(), name, ticker, 0, Some(self.callbacks().wood_token_callback()));
    }

    #[storage_mapper("ticker")]
    fn token_manager(&self) -> FungibleTokenMapper<Self::Api>;

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