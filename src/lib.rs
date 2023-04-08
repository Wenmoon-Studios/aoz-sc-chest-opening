#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod owner;
pub mod storage;
pub mod context;
use context::StorageCache;

#[multiversx_sc::contract]
pub trait AozScLandChestOpening: storage::StorageModule + owner::OwnerModule {
    #[init]
    fn init(
        &self,
        is_contract_disabled: OptionalValue<bool>,
        chest_token_id_opt: OptionalValue<TokenIdentifier>
    ) {
        match is_contract_disabled {
            OptionalValue::Some(val) => self.enabled().set(&val),
            OptionalValue::None => self.enabled().set(false)
        };
        match chest_token_id_opt {
            OptionalValue::Some(val) => self.chest_token_id().set(val),
            OptionalValue::None => {
                require!(
                    !self.chest_token_id().is_empty(),
                    "chest token id was not supplied"
                );
            }
        }
        for chest_nonce in &[1, 2, 3, 4u64] {
            for item in self.chance_based_item_set(*chest_nonce).iter() {
                let (drop, _) = item;
                let sc_balance = self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::esdt(drop.clone().token_identifier), drop.clone().token_nonce);
                if sc_balance > 0 {
                    continue;
                }
                self.chance_based_item_set(*chest_nonce).remove(&drop);
            }
        }
    }

    #[payable("*")]
    #[endpoint(openChests)]
    fn open_chests(&self) {
        require!(!self.enabled().is_empty(), "maintenance");

        let payment = self.call_value().all_esdt_transfers();
        require!(payment.len() > 0usize, "no chests sent");

        let chest_token_id = self.chest_token_id().get();
        let mut rewards_vec = ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new();

        for chest in payment.iter() {
            require!(&chest.token_identifier == &chest_token_id, "wrong SFTs sent");
            let mut storage_cache  = StorageCache::new(self, chest.token_nonce);
            let guaranteed_drop = storage_cache.get_guaranteed_drop_with_quantity(chest.amount.clone());
            rewards_vec.push(guaranteed_drop);

            for _ in 0..chest.amount.to_u64().unwrap() as usize {
                if storage_cache.has_won_chance_drop() {
                    let chance_drop = storage_cache.get_chance_drop();
                    rewards_vec.push(chance_drop);
                }
                let guaranteed_drop_from_set = storage_cache.get_guaranteed_drop_from_set();
                rewards_vec.push(guaranteed_drop_from_set);
            }
        }

        require!(!rewards_vec.is_empty(), "no rewards to receive");
        //send back the rewards
        self.send()
            .direct_multi(&self.blockchain().get_caller(), &rewards_vec);
    }
}
