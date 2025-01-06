#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug)]
pub struct ResourceConfig<M: ManagedTypeApi> {
    token_id: TokenIdentifier<M>,
    rounds_required: u64,
}

#[multiversx_sc::contract]
pub trait Resource {
    #[init]
    fn init(&self) {}

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
    fn generate_resources(&self, resource_type: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        let stake = self.stake_amount(&caller).get();
        let current_round = self.blockchain().get_block_round();
        let last_claim = self.last_claim_round(&caller).get();
        
        require!(stake >= 1000u64, "No stake found");
        
        let config = self.resource_configs(&resource_type).get();
        let multiplier = &stake / 1000u64;
        let amount_to_generate = multiplier * config.rounds_required;

        if amount_to_generate > 0 {
            self.pending_resources(&caller, &resource_type).set(&amount_to_generate);
            self.last_claim_round(&caller).set(&current_round);
        }
    }

    #[endpoint(claimResources)]
    fn claim_resources(&self, resource_type: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        let amount = self.pending_resources(&caller, &resource_type).get();
        require!(amount > 0, "No resources to claim");

        let config = self.resource_configs(&resource_type).get();
        
        self.send()
            .esdt_local_mint(&config.token_id, 0, &amount);
        self.send()
            .direct_esdt(
                &caller,
                &config.token_id,
                0,
                &amount
            );
        
        self.pending_resources(&caller, &resource_type).clear();
    }

    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        resource_type: ManagedBuffer,
        name: ManagedBuffer,
        ticker: ManagedBuffer
    ) {
        require!(
            self.resource_configs(&resource_type).is_empty(),
            "Resource type already exists"
        );

        let food_buffer = ManagedBuffer::from(b"FOOD");
        let wood_buffer = ManagedBuffer::from(b"WOOD");
        let stone_buffer = ManagedBuffer::from(b"STONE");
        let gold_buffer = ManagedBuffer::from(b"GOLD");

        let rounds_required = if resource_type.eq(&food_buffer) {
            1u64
        } else if resource_type.eq(&wood_buffer) {
            1u64
        } else if resource_type.eq(&stone_buffer) {
            1u64
        } else if resource_type.eq(&gold_buffer) {
            2u64
        } else {
            sc_panic!("Invalid resource type")
        };

        let amount = self.call_value().egld_value();
        self.token_manager().issue_and_set_all_roles(
            amount.clone_value(),
            name,
            ticker,
            0,
            Some(self.callbacks().token_callback(resource_type, rounds_required))
        );
    }

    #[callback]
    fn token_callback(
        &self,
        resource_type: ManagedBuffer,
        rounds_required: u64,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                let config = ResourceConfig {
                    token_id,
                    rounds_required,
                };
                self.resource_configs(&resource_type).set(&config);
            },
            ManagedAsyncCallResult::Err(_) => sc_panic!("Token issue failed")
        }
    }

    #[storage_mapper("resourceConfigs")]
    fn resource_configs(&self, resource_type: &ManagedBuffer) -> SingleValueMapper<ResourceConfig<Self::Api>>;

    #[storage_mapper("tokenManager")]
    fn token_manager(&self) -> FungibleTokenMapper<Self::Api>;

    #[storage_mapper("stakeAmount")]
    fn stake_amount(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("lastClaimRound")]
    fn last_claim_round(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("pendingResources")]
    fn pending_resources(&self, addr: &ManagedAddress, resource_type: &ManagedBuffer) -> SingleValueMapper<BigUint>;
}