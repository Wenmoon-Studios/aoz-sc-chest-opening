use aoz_sc_land_chest_opening::*;
use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::DebugApi;

#[test]
fn test_add() {
    let _ = DebugApi::dummy();

    let lib = aoz_sc_land_chest_opening::contract_obj::<DebugApi>();

    lib.init(BigUint::from(5u32));
    assert_eq!(BigUint::from(5u32), lib.sum().get());

    lib.add(BigUint::from(7u32));
    assert_eq!(BigUint::from(12u32), lib.sum().get());

    lib.add(BigUint::from(1u32));
    assert_eq!(BigUint::from(13u32), lib.sum().get());
}
