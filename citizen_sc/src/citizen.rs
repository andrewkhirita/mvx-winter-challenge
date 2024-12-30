#![no_std]
use multiversx_sc::imports::*;

const BLOCKS_IN_HOUR: u64 = 600;
const WOOD_REQUIRED: u64 = 10;
const FOOD_REQUIRED: u64 = 15;

#[multiversx_sc::contract]
pub trait Citizen {
    #[init]
    fn init(&self) {
        self.issue_citizen_token();
    }

    #[payable("*")]
    #[endpoint(mintCitizen)]
    fn mint_citizen(&self) {
        let payments = self.call_value().all_esdt_transfers();
        let caller = self.blockchain().get_caller();
        let mut wood_amount = BigUint::zero();
        let mut food_amount = BigUint::zero();

        for payment in payments.iter() {
            match payment.token_identifier.as_managed_buffer().to_boxed_bytes().as_slice() {
                b"WOOD" => wood_amount += &payment.amount,
                b"FOOD" => food_amount += &payment.amount,
                _ => sc_panic!("Invalid token")
            }
        }

        require!(wood_amount >= WOOD_REQUIRED, "Not enough WOOD");
        require!(food_amount >= FOOD_REQUIRED, "Not enough FOOD");

        // Burn the resources
        self.send().esdt_local_burn(&TokenIdentifier::from_esdt_bytes(b"WOOD"), 0, &wood_amount);
        self.send().esdt_local_burn(&TokenIdentifier::from_esdt_bytes(b"FOOD"), 0, &food_amount);

        // Store mint request
        let claim_block = self.blockchain().get_block_nonce() + BLOCKS_IN_HOUR;
        self.pending_claims(&caller).set(&claim_block);
    }

    #[endpoint(claimCitizen)]
    fn claim_citizen(&self) {
        let caller = self.blockchain().get_caller();
        let claim_block = self.pending_claims(&caller).get();
        let current_block = self.blockchain().get_block_nonce();

        require!(current_block >= claim_block, "Too early to claim");
        require!(!self.citizen_token_id().is_empty(), "Token not issued yet");

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

        // Send the NFT to the caller
        self.send().direct_esdt(
            &caller,
            &self.citizen_token_id().get(),
            nonce,
            &BigUint::from(1u64)
        );

        self.next_nonce().update(|val| *val += 1);
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

    fn issue_citizen_token(&self) {
        self.send()
            .esdt_system_sc_proxy()
            .issue_non_fungible(
                &ManagedBuffer::from(b"CITIZEN"),
                &ManagedBuffer::from(b"CITIZEN"),
                &BigUint::zero(),
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                    can_transfer_create_role: true
                }
            )
            .with_callback(self.callbacks().citizen_token_callback())
            .async_call_and_exit()
    }

    #[storage_mapper("citizenTokenId")]
    fn citizen_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("nextNonce")]
    fn next_nonce(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("pendingClaims")]
    fn pending_claims(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;
}