#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// const BLOCKS_IN_HOUR: u64 = 600;
const BLOCKS_IN_HOUR: u64 = 1;

const WOOD_REQUIRED: u64 = 10;
const FOOD_REQUIRED: u64 = 15;

const GOLD_REQUIRED: u64 = 5;
const ORE_REQUIRED: u64 = 5;


#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug)]
pub struct CharacterAttributes<M: ManagedTypeApi> {
    rank: ManagedBuffer<M>,
    defense: u8,
    attack: u8,
}

#[multiversx_sc::contract]
pub trait Citizen {
    #[init]
    fn init(&self) {
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(mintCitizen)]
    fn mint_citizen(&self) {
        let (wood_amount, food_amount) = self.get_and_validate_resources();
        let caller = self.blockchain().get_caller();
        
        self.burn_resources(wood_amount, food_amount);
        
        let claim_block = self.blockchain().get_block_nonce() + BLOCKS_IN_HOUR;
        self.pending_claims(&caller).set(&claim_block);
    }

    #[endpoint(claimCitizen)]
    fn claim_citizen(&self) {
        let caller = self.blockchain().get_caller();
        require!(self.is_claim_ready(&caller), "Too early to claim");
        
        self.mint_and_send_nft(&caller);
        self.pending_claims(&caller).clear();
    }

    #[callback]
    fn citizen_token_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.citizen_token_id().set(&token_id);
                self.next_nonce().set(&1u64);
            },
            ManagedAsyncCallResult::Err(_) => sc_panic!("Token issue failed")
        }
    }

    #[payable("EGLD")]
    #[endpoint(issue)]
    fn issue_citizen_token(&self, name: ManagedBuffer, ticker: ManagedBuffer) {
        let amount = self.call_value().egld_value();
        self.token_manager().issue_and_set_all_roles(EsdtTokenType::NonFungible, amount.clone_value(), name, ticker, 0, Some(self.callbacks().citizen_token_callback()));
    }

    fn get_and_validate_resources(&self) -> (BigUint, BigUint) {
        let mut wood = BigUint::zero();
        let mut food = BigUint::zero();
        
        for payment in self.call_value().all_esdt_transfers().iter() {
            let token_buffer = payment.token_identifier.as_managed_buffer();
            if token_buffer.len() >= 5 {
                let prefix = token_buffer.copy_slice(0, 5).unwrap();
                if prefix == ManagedBuffer::from(b"WOOD-") {
                    wood += &payment.amount;
                } else if prefix == ManagedBuffer::from(b"FOOD-") {
                    food += &payment.amount;
                } else {
                    sc_panic!("Invalid token");
                }
            } else {
                sc_panic!("Invalid token identifier length");
            }
        }
        
        require!(wood >= WOOD_REQUIRED, "Not enough WOOD");
        require!(food >= FOOD_REQUIRED, "Not enough FOOD");
        (wood, food)
    }
    
    fn burn_resources(&self, wood: BigUint, food: BigUint) {
        let payments = self.call_value().all_esdt_transfers();
        let mut wood_token_id = None;
        let mut food_token_id = None;
        
        for payment in payments.iter() {
            let token_buffer = payment.token_identifier.as_managed_buffer();
            if token_buffer.len() >= 5 {
                let prefix = token_buffer.copy_slice(0, 5).unwrap();
                if prefix == ManagedBuffer::from(b"WOOD-") {
                    wood_token_id = Some(payment.token_identifier.clone());
                } else if prefix == ManagedBuffer::from(b"FOOD-") {
                    food_token_id = Some(payment.token_identifier.clone());
                }
            }
        }
        
        if let Some(wood_id) = wood_token_id {
            self.send().esdt_local_burn(&wood_id, 0, &wood);
        }
        if let Some(food_id) = food_token_id {
            self.send().esdt_local_burn(&food_id, 0, &food);
        }
    }

    fn is_claim_ready(&self, caller: &ManagedAddress) -> bool {
        let claim_block = self.pending_claims(caller).get();
        self.blockchain().get_block_nonce() >= claim_block
    }

    fn mint_and_send_nft(&self, to: &ManagedAddress) {
        let nonce = self.next_nonce().get();
        
        self.send().esdt_nft_create(
            &self.citizen_token_id().get(),
            &BigUint::from(1u64),
            &ManagedBuffer::from(b"CITIZEN"),
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &Empty,
            &ManagedVec::new(),
        );

        self.send().direct_esdt(
            to,
            &self.citizen_token_id().get(),
            nonce,
            &BigUint::from(1u64)
        );

        self.next_nonce().update(|val| *val += 1);
    }

    #[payable("*")]
    #[endpoint(upgradeCitizen)]
    fn upgrade_citizen(
        &self
    ) {
        let payments = self.call_value().all_esdt_transfers();
        require!(payments.len() == 3, "Expected 3 tokens");

        let citizen_payment = payments.get(0);
        require!(
            citizen_payment.token_identifier == self.citizen_token_id().get(),
            "Invalid Citizen NFT"
        );
        
        let (gold_amount, ore_amount) = self.get_and_validate_upgrade_resources();
        let caller = self.blockchain().get_caller();
        
        self.burn_upgrade_resources(gold_amount, ore_amount);
        
        self.upgrade_nft_to_soldier(&caller, citizen_payment.token_nonce);
    }

    fn get_and_validate_upgrade_resources(&self) -> (BigUint, BigUint) {
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
                }
            }
        }
        
        require!(gold >= GOLD_REQUIRED, "Not enough GOLD");
        require!(ore >= ORE_REQUIRED, "Not enough ORE");
        (gold, ore)
    }
    
    fn burn_upgrade_resources(&self, gold: BigUint, ore: BigUint) {
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

    fn upgrade_nft_to_soldier(&self, to: &ManagedAddress, citizen_nonce: u64) {
        let mut attributes = ManagedBuffer::new();
        attributes.append(&ManagedBuffer::from(b"rank: soldier\ndefense: 0\nattack: 0"));
        
        self.send().nft_update_attributes(
            &self.citizen_token_id().get(),
            citizen_nonce,
            &attributes,
        );
    
        self.send().direct_esdt(
            to,
            &self.citizen_token_id().get(),
            citizen_nonce,
            &BigUint::from(1u64)
        );
    }

    #[payable("*")]
    #[endpoint(equipShield)]
    fn equip_shield(&self) {
        let payments = self.call_value().all_esdt_transfers();
        require!(payments.len() == 2, "Expected 2 tokens");

        let citizen_payment = payments.get(0);
        let shield_payment = payments.get(1);
        
        require!(
            citizen_payment.token_identifier == self.citizen_token_id().get(),
            "Invalid Citizen NFT"
        );

        self.validate_soldier_and_shield(&citizen_payment, &shield_payment);
        
        let caller = self.blockchain().get_caller();
        
        self.update_character_with_shield(&caller, citizen_payment.token_nonce);
    }

    #[payable("*")]
    #[endpoint(equipSword)]
    fn equip_sword(&self) {
        let payments = self.call_value().all_esdt_transfers();
        require!(payments.len() == 2, "Expected 2 tokens");

        let citizen_payment = payments.get(0);
        let sword_payment = payments.get(1);
        
        require!(
            citizen_payment.token_identifier == self.citizen_token_id().get(),
            "Invalid Citizen NFT"
        );

        self.validate_soldier_and_sword(&citizen_payment, &sword_payment);
        
        let caller = self.blockchain().get_caller();
        self.update_character_with_sword(&caller, citizen_payment.token_nonce);
    }

    fn validate_soldier_and_shield(
        &self,
        citizen_payment: &EsdtTokenPayment<Self::Api>,
        shield_payment: &EsdtTokenPayment<Self::Api>
    ) {
        let token_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &citizen_payment.token_identifier,
            citizen_payment.token_nonce,
        );
    
        let soldier_prefix = ManagedBuffer::from(b"rank: soldier");
        let prefix_length = soldier_prefix.len();
        let attributes = token_data.attributes.clone();
        
        require!(
            attributes.len() >= prefix_length && 
            attributes.copy_slice(0, prefix_length).unwrap() == soldier_prefix,
            "Character must be a soldier"
        );
    
        let shield_token_id = shield_payment.token_identifier.as_managed_buffer();
        require!(
            shield_token_id.copy_slice(0, 4).unwrap() == ManagedBuffer::from(b"TOOL"),
            "Invalid Shield NFT"
        );
    }

    fn validate_soldier_and_sword(
        &self,
        citizen_payment: &EsdtTokenPayment<Self::Api>,
        sword_payment: &EsdtTokenPayment<Self::Api>
    ) {
        let token_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &citizen_payment.token_identifier,
            citizen_payment.token_nonce,
        );
    
        let soldier_prefix = ManagedBuffer::from(b"rank: soldier");
        let prefix_length = soldier_prefix.len();
        let attributes = token_data.attributes.clone();
        
        require!(
            attributes.len() >= prefix_length && 
            attributes.copy_slice(0, prefix_length).unwrap() == soldier_prefix,
            "Character must be a soldier"
        );
    
        let sword_token_id = sword_payment.token_identifier.as_managed_buffer();
        require!(
            sword_token_id.copy_slice(0, 4).unwrap() == ManagedBuffer::from(b"TOOL"),
            "Invalid Shield NFT"
        );
    }

    fn update_character_with_shield(&self, to: &ManagedAddress, citizen_nonce: u64) {
        let token_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &self.citizen_token_id().get(),
            citizen_nonce,
        );
    
        let old_attributes = token_data.attributes;
        if old_attributes == ManagedBuffer::from(b"rank: soldier\ndefense: 0\nattack: 0") {
            let mut new_attributes = ManagedBuffer::new();
            new_attributes.append(&ManagedBuffer::from(b"rank: soldier\ndefense: 1\nattack: 0"));
            self.send().nft_update_attributes(
                &self.citizen_token_id().get(),
                citizen_nonce,
                &new_attributes,
            );
        } else if old_attributes == ManagedBuffer::from(b"rank: soldier\ndefense: 1\nattack: 0") {
            let mut new_attributes = ManagedBuffer::new();
            new_attributes.append(&ManagedBuffer::from(b"rank: soldier\ndefense: 2\nattack: 0"));
            self.send().nft_update_attributes(
                &self.citizen_token_id().get(),
                citizen_nonce,
                &new_attributes,
            );
        }
    
        self.send().direct_esdt(
            to,
            &self.citizen_token_id().get(),
            citizen_nonce,
            &BigUint::from(1u64)
        );
    }
    
    fn update_character_with_sword(&self, to: &ManagedAddress, citizen_nonce: u64) {
        let token_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &self.citizen_token_id().get(),
            citizen_nonce,
        );
    
        let old_attributes = token_data.attributes;
        if old_attributes == ManagedBuffer::from(b"rank: soldier\ndefense: 0\nattack: 0") {
            let mut new_attributes = ManagedBuffer::new();
            new_attributes.append(&ManagedBuffer::from(b"rank: soldier\ndefense: 0\nattack: 1"));
            self.send().nft_update_attributes(
                &self.citizen_token_id().get(),
                citizen_nonce,
                &new_attributes,
            );
        } else if old_attributes == ManagedBuffer::from(b"rank: soldier\ndefense: 0\nattack: 1") {
            let mut new_attributes = ManagedBuffer::new();
            new_attributes.append(&ManagedBuffer::from(b"rank: soldier\ndefense: 0\nattack: 2"));
            self.send().nft_update_attributes(
                &self.citizen_token_id().get(),
                citizen_nonce,
                &new_attributes,
            );
        }
    
        self.send().direct_esdt(
            to,
            &self.citizen_token_id().get(),
            citizen_nonce,
            &BigUint::from(1u64)
        );
    }

    #[storage_mapper("citizenTokenId")]
    fn citizen_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("ticker")]
    fn token_manager(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[storage_mapper("nextNonce")]
    fn next_nonce(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("pendingClaims")]
    fn pending_claims(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;
}