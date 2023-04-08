multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait OwnerModule: crate::storage::StorageModule {
    #[only_owner]
    #[endpoint(enableSc)]
    fn enable_sc(&self) {
        self.enabled().set(true);
    }

    #[only_owner]
    #[endpoint(disableSc)]
    fn disable_sc(&self) {
        self.enabled().clear();
    }

    #[only_owner]
    #[endpoint(setGuaranteedItem)]
    fn set_guaranteed_item(&self, chest_nonce: u64, token_id: TokenIdentifier, nonce: u64) {
        self.guaranteed_item(chest_nonce)
            .set(
                &EsdtTokenPayment::new(
                    token_id,
                    nonce,
                    BigUint::from(1u32)
                )
            );
    }

    #[only_owner]
    #[endpoint(addGuaranteedSetItem)]
    fn add_guaranteed_set_item(&self, chest_nonce: u64, token_id: TokenIdentifier, nonce: u64, total_amount: usize) {
        self.guaranteed_item_set(chest_nonce)
            .insert(
                EsdtTokenPayment::new(
                    token_id,
                    nonce,
                    BigUint::from(1u32)
                ), 
                total_amount
            );
    }

    #[only_owner]
    #[endpoint(addChanceSetItem)]
    fn add_chance_set_item(&self, chest_nonce: u64, token_id: TokenIdentifier, nonce: u64, total_amount: usize) {
        self.chance_based_item_set(chest_nonce)
            .insert(
                EsdtTokenPayment::new(
                    token_id,
                    nonce,
                    BigUint::from(1u32)
                ), 
                total_amount
            );
    }

    #[only_owner]
    #[endpoint(clearGuaranteedItem)]
    fn clear_guaranteed_item(&self, chest_nonce: u64) {
        self.guaranteed_item(chest_nonce).clear();
    }

    #[only_owner]
    #[endpoint(clearGuaranteedItemSet)]
    fn clear_guaranteed_item_set(&self, chest_nonce: u64) {
        self.guaranteed_item_set(chest_nonce).clear();
    }

    #[only_owner]
    #[endpoint(clearChanceItemSet)]
    fn clear_chance_item_set(&self, chest_nonce: u64) {
        self.chance_based_item_set(chest_nonce).clear();
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(deposit)]
    fn deposit(&self) {}
}
