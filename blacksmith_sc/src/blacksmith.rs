#![no_std]
use multiversx_sc::imports::*;

const BLOCKS_IN_HOUR: u64 = 600;
const ORE_REQUIRED_FOR_SHIELD: u64 = 2;

const GOLD_REQUIRED_FOR_SWORD: u64 = 1;
const ORE_REQUIRED_FOR_SWORD: u64 = 3;

#[multiversx_sc::contract]
pub trait Blacksmith {
    #[init]
    fn init(&self) {
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(mintShield)]
    fn mint_shield(&self) {
        let ore_amount = self.validate_resources_for_shield();
        let caller = self.blockchain().get_caller();
        
        self.burn_resources_for_shield(ore_amount);
        
        let claim_block = self.blockchain().get_block_nonce() + BLOCKS_IN_HOUR;
        self.pending_claims(&caller).set(&claim_block);
    }

    #[endpoint(claimShield)]
    fn claim_shield(&self) {
        let caller = self.blockchain().get_caller();
        require!(self.is_claim_ready(&caller), "Too early to claim");
        
        self.mint_helper_shield(&caller);
        self.pending_claims(&caller).clear();
    }

    #[payable("*")]
    #[endpoint(mintSword)]
    fn mint_sword(&self) {
        let (gold_amount, ore_amount) = self.validate_resources_for_sword();
        let caller = self.blockchain().get_caller();
        
        self.burn_resources_for_sword(gold_amount,ore_amount);
        
        let claim_block = self.blockchain().get_block_nonce() + BLOCKS_IN_HOUR;
        self.pending_claims(&caller).set(&claim_block);
    }

    #[endpoint(claimSword)]
    fn claim_sword(&self) {
        let caller = self.blockchain().get_caller();
        require!(self.is_claim_ready(&caller), "Too early to claim");
        
        self.mint_helper_sword(&caller);
        self.pending_claims(&caller).clear();
    }

    fn validate_resources_for_shield(&self) -> BigUint{
        let mut ore = BigUint::zero();
        
        for payment in self.call_value().all_esdt_transfers().iter() {
            let token_buffer = payment.token_identifier.as_managed_buffer();
            if token_buffer.len() >= 4 {
                let prefix = token_buffer.copy_slice(0, 4).unwrap();
                if prefix == ManagedBuffer::from(b"ORE-") {
                    ore += &payment.amount;
                } else {
                    sc_panic!("Invalid token");
                }
            } else {
                sc_panic!("Invalid token identifier length");
            }
        }
        
        require!(ore >= ORE_REQUIRED_FOR_SHIELD, "Not enough ORE");
        ore
    }
    
    fn burn_resources_for_shield(&self, ore: BigUint) {
        let payments = self.call_value().all_esdt_transfers();
        let mut ore_token_id = None;
        
        for payment in payments.iter() {
            let token_buffer = payment.token_identifier.as_managed_buffer();
            if token_buffer.len() >= 5 {
                let prefix = token_buffer.copy_slice(0, 4).unwrap();
                if prefix == ManagedBuffer::from(b"ORE-") {
                    ore_token_id = Some(payment.token_identifier.clone());
                }
            }
        }
        
        if let Some(ore_id) = ore_token_id {
            self.send().esdt_local_burn(&ore_id, 0, &ore);
        }
    }

    fn validate_resources_for_sword(&self) -> (BigUint, BigUint) {
        let mut gold = BigUint::zero();
        let mut ore = BigUint::zero();
        
        for payment in self.call_value().all_esdt_transfers().iter() {
            let token_buffer = payment.token_identifier.as_managed_buffer();
            if token_buffer.len() >= 5 {
                let prefix_gold = token_buffer.copy_slice(0, 5).unwrap();
                let prefix_ore = token_buffer.copy_slice(0, 4).unwrap();
                if prefix_gold == ManagedBuffer::from(b"GOLD-") {
                    gold += &payment.amount;
                } else if prefix_ore == ManagedBuffer::from(b"ORE-") {
                    ore += &payment.amount;
                } else {
                    sc_panic!("Invalid token");
                }
            } else {
                sc_panic!("Invalid token identifier length");
            }
        }
        
        require!(gold >= GOLD_REQUIRED_FOR_SWORD, "Not enough WOOD");
        require!(ore >= ORE_REQUIRED_FOR_SWORD, "Not enough FOOD");
        (gold, ore)
    }
    
    fn burn_resources_for_sword(&self, gold: BigUint, ore: BigUint) {
        let payments = self.call_value().all_esdt_transfers();
        let mut gold_token_id = None;
        let mut ore_token_id = None;
        
        for payment in payments.iter() {
            let token_buffer = payment.token_identifier.as_managed_buffer();
            if token_buffer.len() >= 5 {
                let prefix_gold = token_buffer.copy_slice(0, 5).unwrap();
                let prefix_ore = token_buffer.copy_slice(0, 4).unwrap();
                if prefix_gold == ManagedBuffer::from(b"GOLD-") {
                    gold_token_id = Some(payment.token_identifier.clone());
                } else if prefix_ore == ManagedBuffer::from(b"ORE-") {
                    ore_token_id = Some(payment.token_identifier.clone());
                }
            }
        }
        
        if let Some(gold_id) = gold_token_id {
            self.send().esdt_local_burn(&gold_id, 0, &gold);
        }
        if let Some(ore_id) = ore_token_id {
            self.send().esdt_local_burn(&ore_id, 0, &ore);
        }
    }

    fn is_claim_ready(&self, caller: &ManagedAddress) -> bool {
        let claim_block = self.pending_claims(caller).get();
        self.blockchain().get_block_nonce() >= claim_block
    }

    fn mint_helper_shield(&self, to: &ManagedAddress) {
        let nonce = self.next_nonce().get();
        
        self.send().esdt_nft_create(
            &self.tool_token_id().get(),
            &BigUint::from(1u64),
            &ManagedBuffer::from(b"SHIELD"),
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &Empty,
            &ManagedVec::new(),
        );

        self.send().direct_esdt(
            to,
            &self.tool_token_id().get(),
            nonce,
            &BigUint::from(1u64)
        );

        self.next_nonce().update(|val| *val += 1);
    }

    fn mint_helper_sword(&self, to: &ManagedAddress) {
        let nonce = self.next_nonce().get();
        
        self.send().esdt_nft_create(
            &self.tool_token_id().get(),
            &BigUint::from(1u64),
            &ManagedBuffer::from(b"SWORD"),
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &Empty,
            &ManagedVec::new(),
        );

        self.send().direct_esdt(
            to,
            &self.tool_token_id().get(),
            nonce,
            &BigUint::from(1u64)
        );

        self.next_nonce().update(|val| *val += 1);
    }

    #[callback]
    fn tool_token_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.tool_token_id().set(&token_id);
                self.next_nonce().set(&1u64);
            },
            ManagedAsyncCallResult::Err(_) => sc_panic!("Token issue failed")
        }
    }

    #[payable("EGLD")]
    #[endpoint(issue)]
    fn issue_tool_token(&self, name: ManagedBuffer, ticker: ManagedBuffer) {
        let amount = self.call_value().egld_value();
        self.token_manager().issue_and_set_all_roles(EsdtTokenType::NonFungible, 
            amount.clone_value(), name, ticker, 0, 
            Some(self.callbacks().tool_token_callback()));
    }

    #[storage_mapper("toolTokenId")]
    fn tool_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("ticker")]
    fn token_manager(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[storage_mapper("nextNonce")]
    fn next_nonce(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("pendingClaims")]
    fn pending_claims(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;
}