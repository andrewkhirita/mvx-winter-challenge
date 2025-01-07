#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, PartialEq)]
pub struct ArenaGame<M: ManagedTypeApi> {
    initiator: ManagedAddress<M>,
    initiator_soldier: EsdtTokenPayment<M>,
    entrance_fee: BigUint<M>, 
    competitor: Option<ManagedAddress<M>>,
    competitor_soldier: Option<EsdtTokenPayment<M>>,
}

#[multiversx_sc::contract]
pub trait Arena {
    #[init]
    fn init(&self) {
        self.next_game_id().set(1u64);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(createGame)]
    fn create_game(&self) {
        let payments = self.call_value().all_esdt_transfers();
        let egld_payment = self.call_value().egld_value();
        
        let soldier = &payments.get(0);
        self.validate_soldier(soldier);

        require!(payments.len() > 1, "Must send soldier NFT and EGLD payment");
        
        let caller = self.blockchain().get_caller();
        let game_id = self.next_game_id().get();

        self.games(&game_id).set(&ArenaGame {
            initiator: caller,
            initiator_soldier: soldier.clone(),
            entrance_fee: payments.get(1).amount.clone(), 
            competitor: None,
            competitor_soldier: None,
        });

        self.next_game_id().update(|id| *id += 1);
    }

    #[payable("*")]
    #[endpoint(joinGame)]
    fn join_game(&self, game_id: u64) {
        let payments = self.call_value().all_esdt_transfers();
        
        let soldier = &payments.get(0);
        self.validate_soldier(soldier);

        require!(payments.len() > 1, "Must send soldier NFT and EGLD payment");
        
        let mut game = self.games(&game_id).get();
        require!(game.competitor.is_none(), "Game already has a competitor");
        require!(
            payments.get(1).amount == game.entrance_fee,
            "Invalid entrance fee"
        );

        let caller = self.blockchain().get_caller();
        game.competitor = Some(caller);
        game.competitor_soldier = Some(soldier.clone());
        
        self.games(&game_id).set(&game);
        self.start_fight(game_id);
    }

    fn start_fight(&self, game_id: u64) {
        let game = self.games(&game_id).get();
        let competitor = game.competitor.as_ref().unwrap();
        
        let initiator_power = self.get_total_power(&game.initiator_soldier);
        let competitor_power = self.get_total_power(game.competitor_soldier.as_ref().unwrap());
        
        let initiator_win_chance = self.calculate_win_chance(initiator_power, competitor_power);
        let random = self.blockchain().get_block_timestamp() % 100;
        let initiator_wins = random < initiator_win_chance;

        if initiator_wins {
            self.send().direct_esdt(
                &game.initiator,
                &game.initiator_soldier.token_identifier,
                game.initiator_soldier.token_nonce,
                &BigUint::from(1u64),
            );
            self.send().direct_egld(
                &game.initiator,
                &game.entrance_fee,  
            );
            self.send().esdt_local_burn(
                &game.competitor_soldier.as_ref().unwrap().token_identifier,
                game.competitor_soldier.as_ref().unwrap().token_nonce,
                &BigUint::from(1u64),
            );
        } else {
            self.send().direct_esdt(
                competitor,
                &game.competitor_soldier.as_ref().unwrap().token_identifier,
                game.competitor_soldier.as_ref().unwrap().token_nonce,
                &BigUint::from(1u64),
            );
            self.send().direct_egld(
                competitor,
                &game.entrance_fee,  
            );
            self.send().esdt_local_burn(
                &game.initiator_soldier.token_identifier,
                game.initiator_soldier.token_nonce,
                &BigUint::from(1u64),
            );
        }

        self.games(&game_id).clear();
    }


    fn validate_soldier(&self, soldier: &EsdtTokenPayment<Self::Api>) {
        let token_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &soldier.token_identifier,
            soldier.token_nonce,
        );

        let attributes = token_data.attributes;
        let soldier_prefix = ManagedBuffer::from(b"rank: soldier");
        require!(
            attributes.len() >= soldier_prefix.len() && 
            attributes.copy_slice(0, soldier_prefix.len()).unwrap() == soldier_prefix,
            "Must be a soldier NFT"
        );
    }

    fn calculate_win_chance(&self, initiator_power: u8, competitor_power: u8) -> u64 {
        let diff = if initiator_power > competitor_power {
            initiator_power - competitor_power
        } else {
            competitor_power - initiator_power
        } as u64;
        
        if diff >= 100 {
            if initiator_power > competitor_power {
                return 100;
            } else {
                return 0;
            }
        }
        
        if initiator_power > competitor_power {
            50 + diff
        } else {
            50 - diff
        }
    }

    fn get_total_power(&self, soldier: &EsdtTokenPayment<Self::Api>) -> u8 {
        let token_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &soldier.token_identifier,
            soldier.token_nonce,
        );
        
        let attributes = token_data.attributes;
        let mut state = 0u8;
        let mut current_value = 0u8;
        let mut defense = 0u8;
        let mut attack = 0u8;
        let mut skip_to_next_line = false;
        
        for i in 0..attributes.len() {
            let mut byte_buf = [0u8; 1];
            if attributes.load_slice(i, &mut byte_buf).is_ok() {
                match byte_buf[0] {
                    b'\n' => {
                        skip_to_next_line = false;
                        if state == 1 {
                            defense = current_value;
                        } else if state == 2 {
                            attack = current_value;
                        }
                        state = 0;
                        current_value = 0;
                    },
                    b':' if !skip_to_next_line => {
                        state = if defense == 0 { 1 } else { 2 };
                    },
                    b'0'..=b'9' if state > 0 => {
                        current_value = byte_buf[0] - b'0';
                    },
                    _ => {
                        if state == 0 {
                            skip_to_next_line = true;
                        }
                    }
                }
            }
        }
        
        defense + attack
    }

    #[view(getGame)]
    fn get_game(&self, game_id: u64) -> Option<ArenaGame<Self::Api>> {
        if !self.games(&game_id).is_empty() {
            Some(self.games(&game_id).get())
        } else {
            None
        }
    }

    #[storage_mapper("games")]
    fn games(&self, game_id: &u64) -> SingleValueMapper<ArenaGame<Self::Api>>;

    #[storage_mapper("nextGameId")]
    fn next_game_id(&self) -> SingleValueMapper<u64>;

    #[view(getNextGameId)]
    fn get_next_game_id(&self) -> u64 {
        self.next_game_id().get()
    }
}