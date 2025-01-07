#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();


#[multiversx_sc::contract]
pub trait Snow {
    #[upgrade]
    fn upgrade(&self) {}

    #[init]
    fn init(&self) {}

    //Challenge 5
    #[payable("EGLD")]
    #[endpoint(issueTokenSnow)]
    fn issue_token(
        &self,
        token_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        let payment_amount = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();
        
        let token_properties = FungibleTokenProperties {
            num_decimals: 8,
            can_freeze: true,
            can_wipe: true,
            can_pause: true,
            can_change_owner: true,
            can_upgrade: true,
            can_add_special_roles: true,
            can_burn: true,
            can_mint: true,
        };

        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                payment_amount.clone_value(),
                &token_name,
                &token_ticker,
                &initial_supply,
                token_properties
            )
            .with_callback(self.callbacks().issue_callback(&caller, initial_supply))
            .async_call_and_exit();
    }


    #[callback]
    fn issue_callback(
        &self,
        caller: &ManagedAddress,
        initial_supply: BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let payments = self.call_value().all_esdt_transfers();
                if !payments.is_empty() {
                    let token_id = payments.get(0).token_identifier;
                    self.user_tokens(caller).insert(token_id.clone());
                    self.token_balance(&token_id).set(&initial_supply);
                    self.issued_tokens_per_user(caller).insert(token_id);
                    self.token_to_claim().set(payments.get(0).token_identifier);
                }
            }
            ManagedAsyncCallResult::Err(_) => {
                self.send().direct_egld(caller, &self.call_value().egld_value());
            }
        }
    }
    
    //Challenge 6
    #[endpoint(burnTokens)]
    fn burn_tokens(
        &self,
        token_identifier: TokenIdentifier,
        amount: BigUint,
    ) {
        let current_balance = self.token_balance(&token_identifier).get();
        require!(
            current_balance >= amount, "Insufficient balance for burning"
        );
        self.token_balance(&token_identifier).set(&(current_balance - &amount));
        self.send().esdt_local_burn(
            &token_identifier,
            0,
            &amount,
        );
    }
   
   //Challenge 7
    #[view(getAllUsersTokens)]
    fn get_all_user_tokens(&self, address: &ManagedAddress) -> MultiValueEncoded<TokenIdentifier> {
        let mut result = MultiValueEncoded::new();
        for token in self.issued_tokens_per_user(address).iter() {
            result.push(token);
        }
        result
    }

    #[view(getAllUserTokenBalances)]
    fn get_all_user_token_balances(
        &self,
        address: &ManagedAddress,
    ) -> MultiValueEncoded<(TokenIdentifier, BigUint)> {
        let caller = self.blockchain().get_caller();
        require!(
            caller == *address,
            "Only the address owner can view all balances"
        );

        let mut result = MultiValueEncoded::new();
        for token in self.issued_tokens_per_user(address).iter() {
            let balance = self.token_balance(&token).get();
            result.push((token, balance));
        }
        result
    }

    #[view(getSingleTokenBalance)]
    fn get_single_token_balance(&self, token_id: TokenIdentifier) -> BigUint {
       self.token_balance(&token_id).get()
    }

    //Challenge 8
    #[endpoint(claimUserTokens)]
    fn claim_user_tokens(&self, token_identifier: TokenIdentifier) {
        let caller = self.blockchain().get_caller();

        let claimable = self.claimable_amount(&caller).get();
        require!(claimable > 0, "No tokens available to claim");

        self.send().direct_esdt(
            &caller,
            &token_identifier,
            0,
            &claimable,
        );

        self.claimable_amount(&caller).clear();
    }

    //Challenge - 9
    #[endpoint(claimTokens)]
    fn claim_tokens(&self) {
        let caller = self.blockchain().get_caller();

        let user_tokens = self.issued_tokens_per_user(&caller);
        require!(!user_tokens.is_empty(), "No tokens found for this address");

        for token_id in user_tokens.iter() {
            let balance = self.token_balance(&token_id).get();
            if balance > BigUint::zero() {
                self.send().direct_esdt(
                    &caller,
                    &token_id,
                    0, 
                    &balance
                );

                self.token_balance(&token_id).clear();
            }
        }
    }

    #[storage_mapper("tokenToClaim")]
    fn token_to_claim(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("claimableAmount")]
    fn claimable_amount(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("issuedTokensPerUser")]
    fn issued_tokens_per_user(&self, user: &ManagedAddress) -> UnorderedSetMapper<TokenIdentifier>;

    #[storage_mapper("tokenBalance")]
    fn token_balance(&self, token_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[storage_mapper("userTokens")]
    fn user_tokens(&self, address: &ManagedAddress) -> UnorderedSetMapper<TokenIdentifier>;
}