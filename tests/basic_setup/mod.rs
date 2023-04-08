use aoz_sc_land_chest_opening::*;
use aoz_sc_land_chest_opening::owner::OwnerModule;
use multiversx_sc_scenario::{whitebox::{BlockchainStateWrapper,ContractObjWrapper}, rust_biguint, managed_token_id, assert_values_eq};
multiversx_sc::imports!();
multiversx_sc::derive_imports!();
use multiversx_sc::types::{Address};
use multiversx_sc_scenario::{
    DebugApi,
};

pub const WASM_PATH: &str = "../output/aoz-sc-land-chest-opening.wasm";
pub const CHEST_TOKEN_ID: &[u8] = b"LANDCHEST-123456";

pub const GUARANTEED_DROP_TOKEN_ID: &[u8] = b"LANDPLOT-123456";
pub const GUARANTEED_SET_TOKEN_ID_1: &[u8] = b"GUARANTEED1-123456";
pub const GUARANTEED_SET_TOKEN_ID_2: &[u8] = b"GUARANTEED2-123456";
pub const GUARANTEED_SET_TOKEN_ID_3: &[u8] = b"GUARANTEED3-123456";
pub const LEGENDARY_DROP_1: &[u8] = b"LEGENDARY1-123456";
pub const LEGENDARY_DROP_2: &[u8] = b"LEGENDARY2-123456";
pub const LEGENDARY_DROP_3: &[u8] = b"LEGENDARY3-123456";
pub const LEGENDARY_DROP_4: &[u8] = b"LEGENDARY4-123456";
pub const LEGENDARY_DROP_5: &[u8] = b"LEGENDARY5-123456";
pub struct ChestOpeningSetup<ChestOpeningObjBuilder>
where
    ChestOpeningObjBuilder: 'static + Copy + Fn() -> aoz_sc_land_chest_opening::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub user_address: Address,
    pub sc_wrapper: ContractObjWrapper<aoz_sc_land_chest_opening::ContractObj<DebugApi>, ChestOpeningObjBuilder>,
}

impl<ChestOpeningObjBuilder> ChestOpeningSetup<ChestOpeningObjBuilder>
where
    ChestOpeningObjBuilder: 'static + Copy + Fn() -> aoz_sc_land_chest_opening::ContractObj<DebugApi>,
{
    pub fn new(builder: ChestOpeningObjBuilder) -> Self {
        let _ = DebugApi::dummy();

        let rust_zero = rust_biguint!(0u64);
        let mut b_mock = BlockchainStateWrapper::new();
        let owner_addr = b_mock.create_user_account(&rust_zero);
        let user_addr = b_mock.create_user_account(&rust_biguint!(1));
        let sc_wrapper = 
            b_mock.create_sc_account(
                &rust_zero,
                Some(&owner_addr),
                builder,
                WASM_PATH
            );
        
        b_mock.set_nft_balance(&user_addr, CHEST_TOKEN_ID, 1, &rust_biguint!(100), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), GUARANTEED_DROP_TOKEN_ID, 1, &rust_biguint!(100), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), GUARANTEED_SET_TOKEN_ID_1, 1, &rust_biguint!(40), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), GUARANTEED_SET_TOKEN_ID_2, 2, &rust_biguint!(30), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), GUARANTEED_SET_TOKEN_ID_3, 3, &rust_biguint!(30), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), LEGENDARY_DROP_1, 1, &rust_biguint!(1), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), LEGENDARY_DROP_2, 2, &rust_biguint!(2), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), LEGENDARY_DROP_3, 3, &rust_biguint!(3), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), LEGENDARY_DROP_4, 4, &rust_biguint!(4), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), LEGENDARY_DROP_5, 5, &rust_biguint!(5), b"");

        b_mock
            .execute_tx(&owner_addr, &sc_wrapper, &rust_zero, |sc| {
                sc.init(
                    OptionalValue::Some(true),
                    OptionalValue::Some(managed_token_id!(CHEST_TOKEN_ID))
                );

                sc.set_guaranteed_item(1, managed_token_id!(GUARANTEED_DROP_TOKEN_ID), 1);
                sc.add_guaranteed_set_item(1, managed_token_id!(GUARANTEED_SET_TOKEN_ID_1), 1, 40);
                sc.add_guaranteed_set_item(1, managed_token_id!(GUARANTEED_SET_TOKEN_ID_2), 2, 30);
                sc.add_guaranteed_set_item(1, managed_token_id!(GUARANTEED_SET_TOKEN_ID_3), 3, 30);

                sc.add_chance_set_item(1, managed_token_id!(LEGENDARY_DROP_1), 1, 1);
                sc.add_chance_set_item(1, managed_token_id!(LEGENDARY_DROP_2), 2, 2);
                sc.add_chance_set_item(1, managed_token_id!(LEGENDARY_DROP_3), 3, 3);
                sc.add_chance_set_item(1, managed_token_id!(LEGENDARY_DROP_4), 4, 4);
                sc.add_chance_set_item(1, managed_token_id!(LEGENDARY_DROP_5), 5, 5);
            })
            .assert_ok();
        ChestOpeningSetup { b_mock: b_mock, owner_address: owner_addr, user_address: user_addr, sc_wrapper: sc_wrapper }
    }

    pub fn open_chests(&mut self, count: u32) {
        self.b_mock
            .execute_esdt_transfer(&self.user_address, &self.sc_wrapper, CHEST_TOKEN_ID, 1, &rust_biguint!(count), |sc| {
                sc.open_chests();
            })
            .assert_ok();
    }

    pub fn check_prize_drop_count(&mut self, expected_min_amount: u32, expected_max_amount: u32) {
        let mut total_prize_count = rust_biguint!(0);
        let all_prizes = vec![
            (GUARANTEED_DROP_TOKEN_ID, 1),
            (GUARANTEED_SET_TOKEN_ID_1, 1),
            (GUARANTEED_SET_TOKEN_ID_2, 2),
            (GUARANTEED_SET_TOKEN_ID_3, 3),
            (LEGENDARY_DROP_1, 1),
            (LEGENDARY_DROP_2, 2),
            (LEGENDARY_DROP_3, 3),
            (LEGENDARY_DROP_4, 4),
            (LEGENDARY_DROP_5, 5),
        ];

        for item in all_prizes.iter() {
            let (token_id, nonce) = item;
            let reward_count = self.b_mock.get_esdt_balance(&self.user_address, token_id, *nonce);
            total_prize_count += reward_count;
        }
        let expected_min = rust_biguint!(expected_min_amount);
        let expected_max = rust_biguint!(expected_max_amount);
        let comparison = expected_min <= total_prize_count && total_prize_count <= expected_max;
        assert_values_eq!(comparison, true);
    }

    pub fn check_0_sc_balance(&mut self) {
        let mut total_prize_count = rust_biguint!(0);
        let all_prizes = vec![
            (GUARANTEED_DROP_TOKEN_ID, 1),
            (GUARANTEED_SET_TOKEN_ID_1, 1),
            (GUARANTEED_SET_TOKEN_ID_2, 2),
            (GUARANTEED_SET_TOKEN_ID_3, 3),
            (LEGENDARY_DROP_1, 1),
            (LEGENDARY_DROP_2, 2),
            (LEGENDARY_DROP_3, 3),
            (LEGENDARY_DROP_4, 4),
            (LEGENDARY_DROP_5, 5),
        ];

        for item in all_prizes.iter() {
            let (token_id, nonce) = item;
            let reward_count = self.b_mock.get_esdt_balance(self.sc_wrapper.address_ref(), token_id, *nonce);
            total_prize_count += reward_count;
        }

        let expected = rust_biguint!(0);
        let comparison = expected == total_prize_count;
        assert_values_eq!(comparison, true);
    }
}