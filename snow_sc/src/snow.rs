#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait Snow {

    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    // 1. Issue tokens endpoint
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        token_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        let payment = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                payment.clone_value(),
                &token_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: 8,
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_change_owner: false,
                    can_upgrade: false,
                    can_add_special_roles: false,
                    can_mint: false,
                    can_burn: true,
                }
            )
            .with_callback(self.callbacks().issue_callback(&caller))
            .async_call_and_exit()
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
                let token_id = payments.get(0).token_identifier;
                self.last_token().set(&token_id);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.send().direct_egld(caller, &self.call_value().egld_value());
            }
        }
    }

    // 2. Burn tokens endpoint
    #[endpoint(burnTokens)]
    fn burn_tokens(
        &self,
        token_id: TokenIdentifier,
        amount: BigUint,
    ) {
        self.send().esdt_local_burn(&token_id, 0, &amount);
    }

    // 3. Query tokens endpoint
    #[view(getTokens)]
    fn get_tokens(&self) -> TokenIdentifier {
        self.last_token().get()
    }

    // 4. Claim tokens endpoint
    #[endpoint(claimTokens)]
    fn claim_tokens(&self) {
        let caller = self.blockchain().get_caller();
        let token_id = self.last_token().get();
        self.send().direct_esdt(&caller, &token_id, 0, &self.call_value().egld_value());
    }

    #[storage_mapper("lastToken")]
    fn last_token(&self) -> SingleValueMapper<TokenIdentifier>;
}