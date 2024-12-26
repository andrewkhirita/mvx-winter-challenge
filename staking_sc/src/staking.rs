#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug)]
pub struct StakingPosition<M: ManagedTypeApi> {
    pub stake_amount: BigUint<M>,
    pub last_claim_timestamp: u64,
    pub stake_epoch: u64,
    pub reward_recipient: Option<ManagedAddress<M>>,
}

#[multiversx_sc::contract]
pub trait Staking {
    #[upgrade]
    fn upgrade(&self) {}
    
    #[init]
    fn init(&self) {
        self.reward_token_id().set(TokenIdentifier::from("SNOW-"));
    }

    #[payable("*")]
    #[endpoint(stakeTokenWinter)]
    fn stake_token_winter(&self) {
        let payment = self.call_value().single_esdt();
        
        let winter_ticker = ManagedBuffer::from(b"WINTER");
        require!(
            payment.token_identifier.ticker().eq(&winter_ticker),
            "Only WINTER tokens can be staked"
        );

        let caller = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();
        let current_timestamp = self.blockchain().get_block_timestamp();

        // Verificăm dacă există deja o poziție
        if !self.staking_position(&caller).is_empty() {
            // Dacă există, actualizăm suma
            self.staking_position(&caller).update(|position| {
                position.stake_amount += payment.amount;
            });
        } else {
            // Dacă nu există, creăm o nouă poziție
            let new_position = StakingPosition {
                stake_amount: payment.amount,
                last_claim_timestamp: current_timestamp,
                stake_epoch: current_epoch,
                reward_recipient: None,
            };
            self.staking_position(&caller).set(new_position);
        }
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();
        let position = self.staking_position(&caller).get();
        
        require!(position.stake_amount > 0, "Nothing staked");
        
        // Comentăm temporar verificările de timp pentru testare
        // require!(
        //     self.blockchain().get_block_epoch() >= position.stake_epoch + 5,
        //     "Staking period not met"
        // );
        // require!(
        //     self.blockchain().get_block_timestamp() >= position.last_claim_timestamp + 86400,
        //     "Can claim once per 24h"
        // );
    
        // Calculăm 1% din stake amount pentru reward
        let reward = &position.stake_amount / 100u64;
        
        if reward > 0 {
            let recipient = position.reward_recipient.unwrap_or_else(|| caller.clone());
            let reward_token = self.reward_token_id().get();
            
            // Mint-uim tokenul SNOW
            self.send().esdt_local_mint(
                &reward_token,
                0,
                &reward
            );
            
            // Trimitem către recipient
            self.send().direct_esdt(
                &recipient,
                &reward_token,
                0,
                &reward
            );
    
            self.staking_position(&caller).update(|pos| {
                pos.last_claim_timestamp = self.blockchain().get_block_timestamp();
            });
        }
    }

    #[endpoint(setRewardRecipient)]
    fn set_reward_recipient(&self, recipient: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        self.staking_position(&caller).update(|position| {
            position.reward_recipient = Some(recipient);
        });
    }

    #[payable("EGLD")]
    #[endpoint(issueTokenSnow)]
    fn issue_token(
        &self,
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

        self.issue_token_event(&caller, &token_ticker, &initial_supply);

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
            .with_callback(self.callbacks().issue_callback(&caller))
            .async_call_and_exit();
    }

    #[callback]
    fn issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_identifier) => {
                self.reward_token_id().set(token_identifier.clone());
                
                let roles = [EsdtLocalRole::Mint];
                self.send()
                    .esdt_system_sc_proxy()
                    .set_special_roles(
                        &self.blockchain().get_sc_address(),
                        &token_identifier,
                        roles.iter().cloned(),
                    )
                    .with_callback(self.callbacks().roles_callback())
                    .async_call_and_exit();
            },
            ManagedAsyncCallResult::Err(_) => {
                // let returned: ManagedRef<'_, <Self as ContractBase>::Api, BigUint<<Self as ContractBase>::Api>> = self.call_value().egld_value();
                // if returned > 0 {
                //     self.send().direct_egld(caller, &returned);
                // }
            },
        }
    }

    #[callback]
    fn roles_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // Rolurile au fost setate cu succes
            },
            ManagedAsyncCallResult::Err(_) => {
                // Handle error
            }
        }
    }

#[event("issue_token")]
fn issue_token_event(
    &self,
    #[indexed] caller: &ManagedAddress,
    #[indexed] token_ticker: &ManagedBuffer,
    initial_supply: &BigUint,
);

    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, addr: &ManagedAddress) -> SingleValueMapper<StakingPosition<Self::Api>>;

    #[storage_mapper("rewardTokenId")]
    fn reward_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}