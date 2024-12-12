#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait Snow {
    #[upgrade]
    fn upgrade(&self) {}

    #[init]
    fn init(&self) {}


    //Challenge - 5
    #[payable("EGLD")]
    #[endpoint(issueTokenSnow)]
    fn issue_token(&self, 
        token_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
        initial_supply: BigUint,
        can_freeze: bool,
        can_wipe: bool,
        can_pause: bool,
        can_add_special_roles: bool,
        can_burn: bool,
        can_mint: bool,
        ) {
        let payment_amount = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();
        
        let token_properties = FungibleTokenProperties {
            num_decimals,
            can_freeze,
            can_wipe,
            can_pause,
            can_change_owner: false, 
            can_upgrade: false,
            can_add_special_roles,
            can_burn,
            can_mint,
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
            .async_call()
            .with_callback(self.callbacks().issue_callback(&caller)).call_and_exit();
    }

    #[callback]
    fn issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let payments = self.call_value().all_esdt_transfers();
                if !payments.is_empty() {
                    let token_id = payments.get(0).token_identifier;
                    self.token_manager(caller).insert(token_id);
                }
            }
            ManagedAsyncCallResult::Err(_) => {
                self.send().direct_egld(caller, &self.call_value().egld_value());
            }
        }
    }

    //Challenge - 6
    #[endpoint(burnTokens)]
    fn burn_tokens(
        &self,
        token_identifier: TokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        let current_balance = self.balance(&token_identifier).get();
        self.balance(&token_identifier)
            .set(&(current_balance - &amount));

        self.send().esdt_local_burn(
            &token_identifier,
            token_nonce,
            &amount,
        );
    }

    #[storage_mapper("tokenManager")]
    fn token_manager(&self, user: &ManagedAddress) -> UnorderedSetMapper<TokenIdentifier>;

    #[storage_mapper("balance")]
    fn balance(&self, token_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

}
