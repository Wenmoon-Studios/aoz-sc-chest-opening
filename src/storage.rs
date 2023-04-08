multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    //SC
    #[view(getEnabled)]
    #[storage_mapper("enabled")]
    fn enabled(&self) -> SingleValueMapper<bool>;

    //COLLECTION
    #[view(getChestTokenId)]
    #[storage_mapper("chestTokenId")]
    fn chest_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    // REWARDS
    #[storage_mapper("guaranteed_item")]
    fn guaranteed_item(&self, chest_nonce: u64) -> SingleValueMapper<EsdtTokenPayment>;

    #[storage_mapper("guaranteed_item_set")]
    fn guaranteed_item_set(&self, chest_nonce: u64) -> MapMapper<EsdtTokenPayment, usize>;

    #[view(getRemainingChanceBasedItems)]
    #[storage_mapper("chance_based_item_set")]
    fn chance_based_item_set(&self, chest_nonce: u64) -> MapMapper<EsdtTokenPayment, usize>;
}
