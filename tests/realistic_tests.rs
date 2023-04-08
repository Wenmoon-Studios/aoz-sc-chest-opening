mod realistic_setup;
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::realistic_setup::*;

#[test]
fn init_test() {
    let _ = ChestOpeningRealisticSetup::new(aoz_sc_land_chest_opening::contract_obj);
}

#[test]
fn open_1_chest_of_each() {
    let mut setup = ChestOpeningRealisticSetup::new(aoz_sc_land_chest_opening::contract_obj);
    setup.open_chests(&[(1,1), (2,1), (3,1), (4,1)]);
    let expected_min_per_chest = 2;
    let expected_max_per_chest = 3;
    setup.check_prize_drop_count(expected_min_per_chest * 4, expected_max_per_chest * 4);
}

#[test]
fn open_1_chest_of_each_twice() {
    let mut setup = ChestOpeningRealisticSetup::new(aoz_sc_land_chest_opening::contract_obj);
    setup.open_chests(&[(1,1), (2,1), (3,1), (4,1), (1,1), (2,1), (3,1), (4,1)]);
    let expected_min_per_chest = 2;
    let expected_max_per_chest = 3;
    setup.check_prize_drop_count(expected_min_per_chest * 8, expected_max_per_chest * 8);
}

#[test]
fn open_all_continental_chests() {
    let mut setup = ChestOpeningRealisticSetup::new(aoz_sc_land_chest_opening::contract_obj);
    setup.open_chests(&[(1, 15579)]);

    let total_rewards_count = 31202;
    setup.check_prize_drop_count(total_rewards_count, total_rewards_count);  
}

#[test]
fn open_all_steepe_chests() {
    let mut setup = ChestOpeningRealisticSetup::new(aoz_sc_land_chest_opening::contract_obj);
    setup.open_chests(&[(2, 10386)]);

    let total_rewards_count = 20924;
    setup.check_prize_drop_count(total_rewards_count, total_rewards_count);  
}

#[test]
fn open_all_panonic_chests() {
    let mut setup = ChestOpeningRealisticSetup::new(aoz_sc_land_chest_opening::contract_obj);
    setup.open_chests(&[(3, 6924)]);

    let total_rewards_count = 14074;
    setup.check_prize_drop_count(total_rewards_count, total_rewards_count);  
}

#[test]
fn open_all_pontic_chests() {
    let mut setup = ChestOpeningRealisticSetup::new(aoz_sc_land_chest_opening::contract_obj);
    setup.open_chests(&[(4, 1731)]);

    let total_rewards_count = 3802;
    setup.check_prize_drop_count(total_rewards_count, total_rewards_count);  
}

#[test]
fn open_all_chests() {
    let mut setup = ChestOpeningRealisticSetup::new(aoz_sc_land_chest_opening::contract_obj);
    setup.open_chests(&[(1, 15579), (2, 10386) ,(3, 6924), (4, 1731)]);

    let total_rewards_count = 70002;
    setup.check_prize_drop_count(total_rewards_count, total_rewards_count);  
    setup.check_0_sc_balance();
}

#[test]
fn open_99_chests_of_each_type() {
    let mut setup = ChestOpeningRealisticSetup::new(aoz_sc_land_chest_opening::contract_obj);
    setup.open_chests(&[(1,99), (2,99), (3,99), (4,99)]);
    let expected_min_per_chest = 2 * 99 * 4;
    let expected_max_per_chest = 3 * 99 * 4;
    setup.check_prize_drop_count(expected_min_per_chest, expected_max_per_chest);
}