mod basic_setup;
mod realistic_setup;
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::basic_setup::*;

#[test]
fn init_test() {
    let _ = ChestOpeningSetup::new(aoz_sc_land_chest_opening::contract_obj);
}

#[test]
fn open_1_chest_of_100() {
    let mut setup = ChestOpeningSetup::new(aoz_sc_land_chest_opening::contract_obj);
    setup.open_chests(1);
    // case pull is winner: 3 items
    // case pull is loser: 2 items
    setup.check_prize_drop_count(2, 3);
}

#[test]
fn open_10_chests_of_100() {
    let mut setup = ChestOpeningSetup::new(aoz_sc_land_chest_opening::contract_obj);
    // min drops per chest = 2
    // max drops per chest = 3, up to 15 chests
    // => total drop count for 10 chests is in [20, 30]
    setup.open_chests(10);
    setup.check_prize_drop_count(20, 30);
}

#[test]
fn open_50_chests_of_100() {
    let mut setup = ChestOpeningSetup::new(aoz_sc_land_chest_opening::contract_obj);
    // min drops per chest = 2
    // max drops per chest = 3, up to 15 chests
    // => min drop for 50 chests = 100;
    // => max drop for 50 chests = 115 (15 * 3 items + 35 * 2 items)
    // => total drop count for 50 chests is in [100, 115]
    setup.open_chests(50);
    setup.check_prize_drop_count(100, 115);
}

#[test]
fn open_100_chests_of_100() {
    let mut setup = ChestOpeningSetup::new(aoz_sc_land_chest_opening::contract_obj);
    // total drop size is 100
    // this means that with all the chests dropped, all the rewards should've been distributed
    // => total drop count for 100 chests must be equal to 215 (15 * 3 items + 85 * 2 items)
    setup.open_chests(100);
    setup.check_prize_drop_count(215, 215);
    setup.check_0_sc_balance();
}