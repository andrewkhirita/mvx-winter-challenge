#![no_std]
use multiversx_sc::imports::*;

const STONE_REQUIRED: u64 = 20;

#[multiversx_sc::contract]
pub trait Ore {
    #[init]
    fn init(&self) {}

    #[payable("STONE")]
    #[endpoint(convertToOre)]
    fn convert_to_ore(&self) {
        let payment = self.call_value().egld_or_single_esdt();
        let caller = self.blockchain().get_caller();
        
        require!(payment.amount >= STONE_REQUIRED, "Need 20 STONE");
        
        // Burn STONE tokens
        self.send().esdt_local_burn(
            &payment.token_identifier,
            0,
            &payment.amount
        );
        
        // Store pending ORE amount
        let ore_amount = payment.amount / STONE_REQUIRED;
        self.pending_ore(&caller).set(&ore_amount);
    }
    
    #[endpoint(claimOre)]
    fn claim_ore(&self) {
        let caller = self.blockchain().get_caller();
        let ore_amount = self.pending_ore(&caller).get();
        require!(ore_amount > 0, "No ORE to claim");
        
        // Send ORE tokens
        self.send().direct_esdt(
            &caller,
            &TokenIdentifier::from_esdt_bytes(b"ORE"),
            0,
            &ore_amount
        );
        
        self.pending_ore(&caller).clear();
    }

    #[storage_mapper("pendingOre")]
    fn pending_ore(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;
}