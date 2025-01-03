#![no_std]
use multiversx_sc::imports::*;

const STONE_REQUIRED: u64 = 20;

#[multiversx_sc::contract]
pub trait Ore {
    #[init]
    fn init(&self) {
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(mintOre)]
    fn mint_ore(&self) {
        let stone_amount = self.get_and_validate_resources();
        let caller = self.blockchain().get_caller();
        
        self.burn_resources(stone_amount);

        let claim_block = self.blockchain().get_block_nonce() + 1u64; 
        self.pending_claims(&caller).set(&claim_block);
        
    }

    #[endpoint(claimOre)]
    fn claim_ore(&self) {
        let caller = self.blockchain().get_caller();
        
        require!(self.pending_claims(&caller).is_empty() == false, "No pending claim");
        
        let claim_block = self.pending_claims(&caller).get();
        require!(self.blockchain().get_block_nonce() >= claim_block, "Too early to claim");
        
        self.mint_helper(&caller);
        
        self.pending_claims(&caller).clear();
    }

    #[callback]
    fn ore_token_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.ore_token_id().set(&token_id);
                self.next_nonce().set(&1u64);
            },
            ManagedAsyncCallResult::Err(_) => sc_panic!("Token issue failed")
        }
    }

    #[payable("EGLD")]
    #[endpoint(issue)]
    fn issue_token(&self, name: ManagedBuffer, ticker: ManagedBuffer) {
        let amount = self.call_value().egld_value();
        self.token_manager().issue_and_set_all_roles(amount.clone_value(), name, ticker, 0, Some(self.callbacks().ore_token_callback()));
    }

    fn get_and_validate_resources(&self) -> BigUint {
        let mut stone = BigUint::zero();
        
        for payment in self.call_value().all_esdt_transfers().iter() {
            let token_buffer = payment.token_identifier.as_managed_buffer();
            if token_buffer.len() >= 6 {
                let prefix = token_buffer.copy_slice(0, 6).unwrap();
                if prefix == ManagedBuffer::from(b"STONE-") {
                    stone += &payment.amount;
                } else {
                    sc_panic!("Invalid token");
                }
            } else {
                sc_panic!("Invalid token identifier length");
            }
        }
        
        require!(stone >= STONE_REQUIRED, "Not enough STONE");
        stone
    }
    
    fn burn_resources(&self, stone: BigUint) {
        let payments = self.call_value().all_esdt_transfers();
        let mut stone_token_id = None;
        
        for payment in payments.iter() {
            let token_buffer = payment.token_identifier.as_managed_buffer();
            if token_buffer.len() >= 6 {
                let prefix = token_buffer.copy_slice(0, 6).unwrap();
                if prefix == ManagedBuffer::from(b"STONE-") {
                    stone_token_id = Some(payment.token_identifier.clone());
                }
            }
        }
        
        match stone_token_id {
            Some(stone_id) => {
                self.send().esdt_local_burn(&stone_id, 0, &stone);
            },
            None => sc_panic!("Stone token not found in payments")
        }
    }

    fn mint_helper(&self, to: &ManagedAddress) {
        let token_id = self.ore_token_id().get();
        let nonce = 0; 
        
        self.send().esdt_local_mint(
            &token_id,
            nonce,
            &BigUint::from(1000u64)
        );
    
        self.send().direct_esdt(
            to,
            &token_id,
            nonce,
            &BigUint::from(1000u64)
        );
    }

    #[storage_mapper("oreTokenId")]
    fn ore_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("ticker")]
    fn token_manager(&self) -> FungibleTokenMapper<Self::Api>;

    #[storage_mapper("nextNonce")]
    fn next_nonce(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("pendingClaims")]
    fn pending_claims(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;
}