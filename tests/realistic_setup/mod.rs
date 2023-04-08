use aoz_sc_land_chest_opening::*;
use aoz_sc_land_chest_opening::owner::OwnerModule;
use multiversx_sc_scenario::{whitebox::{BlockchainStateWrapper,ContractObjWrapper, TxTokenTransfer}, rust_biguint, managed_token_id, assert_values_eq};
multiversx_sc::imports!();
multiversx_sc::derive_imports!();
use multiversx_sc::types::{Address};
use multiversx_sc_scenario::{
    DebugApi
};

pub const WASM_PATH: &str = "../output/aoz-sc-land-chest-opening.wasm";
pub const CHEST_TOKEN_ID: &[u8] = b"LANDCHEST-123456";

pub const LANDTOKEN_ID: &[u8] = b"LANDTOKENS-123456";
pub const WEAPONTOKEN_ID: &[u8] = b"WTOKENS-123456";
pub const ARMORTOKEN_ID: &[u8] = b"ATOKENS-123456";
pub const MISCTOKEN_ID: &[u8] = b"MTOKENS-123456";
pub const COLLECTIBLE_TOKEN_ID: &[u8] = b"CTOKENS-123456";

pub struct ChestOpeningRealisticSetup<ChestOpeningObjBuilder>
where
    ChestOpeningObjBuilder: 'static + Copy + Fn() -> aoz_sc_land_chest_opening::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub user_address: Address,
    pub sc_wrapper: ContractObjWrapper<aoz_sc_land_chest_opening::ContractObj<DebugApi>, ChestOpeningObjBuilder>,
}

impl<ChestOpeningObjBuilder> ChestOpeningRealisticSetup<ChestOpeningObjBuilder>
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

        let guaranteed_tokens = Self::get_guaranteed_drops();
        let guaranteed_set_tokens = Self::get_guaranteed_set_drops();
        let chance_drops = Self::get_chance_set_drops();

        b_mock.set_nft_balance(&user_addr, CHEST_TOKEN_ID, 1, &rust_biguint!(15579), b"");
        b_mock.set_nft_balance(&user_addr, CHEST_TOKEN_ID, 2, &rust_biguint!(10386), b"");
        b_mock.set_nft_balance(&user_addr, CHEST_TOKEN_ID, 3, &rust_biguint!(6924), b"");
        b_mock.set_nft_balance(&user_addr, CHEST_TOKEN_ID, 4, &rust_biguint!(1731), b"");

        for item in guaranteed_tokens.iter() {
            let (_, reward_nonce, reward_token_id, amount) = item;
            b_mock.set_nft_balance(sc_wrapper.address_ref(), reward_token_id, *reward_nonce, &rust_biguint!(*amount), b"");

        }

        for item in guaranteed_set_tokens.iter() {
            let (_, reward_nonce, reward_token_id, amount) = item;
            b_mock.set_nft_balance(sc_wrapper.address_ref(), reward_token_id, *reward_nonce, &rust_biguint!(*amount), b"");
        }

        for item in chance_drops.iter() {
            let (_, reward_nonce, reward_token_id, amount) = item;
            if reward_token_id == &LANDTOKEN_ID {
                continue;
            }
            b_mock.set_nft_balance(sc_wrapper.address_ref(), reward_token_id, *reward_nonce, &rust_biguint!(*amount), b"");
        }

        b_mock.set_nft_balance(sc_wrapper.address_ref(), LANDTOKEN_ID, 1, &rust_biguint!(666), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), WEAPONTOKEN_ID, 1, &rust_biguint!(18), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), ARMORTOKEN_ID, 1, &rust_biguint!(36), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), MISCTOKEN_ID, 1, &rust_biguint!(12), b"");
        b_mock.set_nft_balance(sc_wrapper.address_ref(), COLLECTIBLE_TOKEN_ID, 1, &rust_biguint!(6), b"");

        b_mock
            .execute_tx(&owner_addr, &sc_wrapper, &rust_zero, |sc| {
                sc.init(
                    OptionalValue::Some(true),
                    OptionalValue::Some(managed_token_id!(CHEST_TOKEN_ID))
                );

                for item in guaranteed_tokens.iter() {
                    let (chest_nonce, reward_nonce, reward_token_id, _) = item;
                    sc.set_guaranteed_item(*chest_nonce, managed_token_id!(*reward_token_id), *reward_nonce);
                }

                for item in guaranteed_set_tokens.iter() {
                    let (chest_nonce, reward_nonce, reward_token_id, amount) = item;
                    sc.add_guaranteed_set_item(*chest_nonce, managed_token_id!(*reward_token_id), *reward_nonce, *amount as usize);
                }

                for item in chance_drops.iter() {
                    let (chest_nonce, reward_nonce, reward_token_id, amount) = item;
                    sc.add_chance_set_item(*chest_nonce, managed_token_id!(*reward_token_id), *reward_nonce, *amount as usize);
                }
            })
            .assert_ok();
        ChestOpeningRealisticSetup { b_mock: b_mock, owner_address: owner_addr, user_address: user_addr, sc_wrapper: sc_wrapper }
    }

    pub fn open_chests(&mut self, transfers: &[(u64, u64)]) {
        let mut tx_transfers = vec![];
        for transfer in transfers.iter() {
            let (nonce, qty) = transfer;
            tx_transfers.push(
                TxTokenTransfer {
                    token_identifier: CHEST_TOKEN_ID.to_vec(),
                    nonce: *nonce,
                    value: rust_biguint!(*qty),
                }
            )
        }
        self.b_mock
            .execute_esdt_multi_transfer(&self.user_address, &self.sc_wrapper, &tx_transfers, |sc| {
                sc.open_chests();
            })
            .assert_ok();
    }

    pub fn check_prize_drop_count(&mut self, expected_min_amount: u32, expected_max_amount: u32) {
        let tokens = &[LANDTOKEN_ID, WEAPONTOKEN_ID, ARMORTOKEN_ID, MISCTOKEN_ID, COLLECTIBLE_TOKEN_ID];
        let mut total_prize_count = rust_biguint!(0);

        for token in tokens.iter() {
            for nonce in [1,2,3,4,5].iter() {
                let reward_count = self.b_mock.get_esdt_balance(&self.user_address, token, *nonce);
                total_prize_count += reward_count;
            }
        }

        let expected_min = rust_biguint!(expected_min_amount);
        let expected_max = rust_biguint!(expected_max_amount);
        let comparison = expected_min <= total_prize_count && total_prize_count <= expected_max;
        assert_values_eq!(comparison, true);
    }

    pub fn check_0_sc_balance(&mut self) {
        let mut total_prize_count = rust_biguint!(0);
        let tokens = &[LANDTOKEN_ID, WEAPONTOKEN_ID, ARMORTOKEN_ID, MISCTOKEN_ID, COLLECTIBLE_TOKEN_ID];


        for token_id in tokens.iter() {
            for nonce in [1,2,3,4,5].iter() {
                let reward_count = self.b_mock.get_esdt_balance(self.sc_wrapper.address_ref(), token_id, *nonce);
                total_prize_count += reward_count;
            }
        }

        let expected = rust_biguint!(0);
        // let comparison = expected == total_prize_count;
        assert_values_eq!(expected, total_prize_count);
    }

    fn get_guaranteed_drops() -> Vec<(u64, u64, &'static [u8], u64)> {
        vec![
            (1, 5, LANDTOKEN_ID, 15579),
            (2, 4, LANDTOKEN_ID, 10386),
            (3, 3, LANDTOKEN_ID, 6924),
            (4, 2, LANDTOKEN_ID, 1731),
        ]
    }

    fn get_guaranteed_set_drops() -> Vec<(u64, u64, &'static [u8], u64)> {
        vec![
            (1, 5, WEAPONTOKEN_ID, 4248),
            (2, 4, WEAPONTOKEN_ID, 2830),
            (3, 3, WEAPONTOKEN_ID, 1887),
            (4, 2, WEAPONTOKEN_ID, 475),

            (1, 5, ARMORTOKEN_ID, 8499),
            (2, 4, ARMORTOKEN_ID, 5668),
            (3, 3, ARMORTOKEN_ID, 3779),
            (4, 2, ARMORTOKEN_ID, 942),

            (1, 5, MISCTOKEN_ID, 2832),
            (2, 4, MISCTOKEN_ID, 1888),
            (3, 3, MISCTOKEN_ID, 1258),
            (4, 2, MISCTOKEN_ID, 314),
        ]
    }
    fn get_chance_set_drops() -> Vec<(u64, u64, &'static [u8], u64)> {
        vec![
            (1, 1, LANDTOKEN_ID, 33),
            (2, 1, LANDTOKEN_ID, 133),
            (3, 1, LANDTOKEN_ID, 200),
            (4, 1, LANDTOKEN_ID, 300),
        
            (1, 1, WEAPONTOKEN_ID, 1),
            (2, 1, WEAPONTOKEN_ID, 3),
            (3, 1, WEAPONTOKEN_ID, 5),
            (4, 1, WEAPONTOKEN_ID, 9),
        
            (1, 1, ARMORTOKEN_ID, 2),
            (2, 1, ARMORTOKEN_ID, 7),
            (3, 1, ARMORTOKEN_ID, 10),
            (4, 1, ARMORTOKEN_ID, 17),
        
            (1, 1, MISCTOKEN_ID, 1),
            (2, 1, MISCTOKEN_ID, 2),
            (3, 1, MISCTOKEN_ID, 3),
            (4, 1, MISCTOKEN_ID, 6),
        
            (1, 1, COLLECTIBLE_TOKEN_ID, 1),
            (2, 1, COLLECTIBLE_TOKEN_ID, 1),
            (3, 1, COLLECTIBLE_TOKEN_ID, 2),
            (4, 1, COLLECTIBLE_TOKEN_ID, 2),
        
            (1, 5, COLLECTIBLE_TOKEN_ID, 6),
            (2, 4, COLLECTIBLE_TOKEN_ID, 6),
            (3, 3, COLLECTIBLE_TOKEN_ID, 6),
            (4, 2, COLLECTIBLE_TOKEN_ID, 6),
        ]
    }

}