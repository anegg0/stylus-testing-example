use alloy_primitives::{address, U256};
use stylus_sdk::testing::*;
use stylus_testing_example::VendingMachine;

#[test]
fn test_give_cupcake_to() {
    // Create a new TestVM
    // let vm = TestVM::default();
    let vm: TestVM = TestVMBuilder::new()
        .sender(address!("dCE82b5f92C98F27F116F70491a487EFFDb6a2a9"))
        .contract_address(address!("0x11b57fe348584f042e436c6bf7c3c3def171de49"))
        .value(U256::from(1))
        .rpc_url("http://localhost:8547")
        .build();

    // Initialize the contract with the VM
    let mut contract = VendingMachine::from(&vm);

    // Test address
    let user = address!("0xCDC41bff86a62716f050622325CC17a317f99404");

    // Check initial balance is zero
    assert_eq!(contract.get_cupcake_balance_for(user).unwrap(), U256::ZERO);

    // Give a cupcake and verify it succeeds
    assert!(contract.give_cupcake_to(user).unwrap());

    // Check balance is now 1
    assert_eq!(
        contract.get_cupcake_balance_for(user).unwrap(),
        U256::from(1)
    );

    // Try to give another cupcake immediately - should fail due to time restriction
    assert!(!contract.give_cupcake_to(user).unwrap());

    // Balance should still be 1
    assert_eq!(
        contract.get_cupcake_balance_for(user).unwrap(),
        U256::from(1)
    );

    // Advance block timestamp by 6 seconds
    vm.set_block_timestamp(vm.block_timestamp() + 6);

    // Now giving a cupcake should succeed
    assert!(contract.give_cupcake_to(user).unwrap());

    // Balance should now be 2
    assert_eq!(
        contract.get_cupcake_balance_for(user).unwrap(),
        U256::from(2)
    );
}

// #[test]
// fn test_multiple_users() {
//     // Create a VM with test addresses using the builder pattern
//     let vm = TestVMBuilder::new()
//         .contract_address(address!("0x11b57fe348584f042e436c6bf7c3c3def171de49"))
//         .build();

//     // Initialize the contract with the VM
//     let mut contract = VendingMachine::from(&vm);

//     // Test addresses
//     let user1 = address!("1111111111111111111111111111111111111111");
//     let user2 = address!("2222222222222222222222222222222222222222");

//     // Give cupcakes to both users
//     assert!(contract.give_cupcake_to(user1).unwrap());
//     assert!(contract.give_cupcake_to(user2).unwrap());

//     // Check both balances are 1
//     assert_eq!(
//         contract.get_cupcake_balance_for(user1).unwrap(),
//         U256::from(1)
//     );
//     assert_eq!(
//         contract.get_cupcake_balance_for(user2).unwrap(),
//         U256::from(1)
//     );

//     // Advance timestamp by 10 seconds
//     vm.set_block_timestamp(vm.block_timestamp() + 10);

//     // Give another cupcake to user1 only
//     assert!(contract.give_cupcake_to(user1).unwrap());

//     // User1 should have 2 cupcakes, user2 still has 1
//     assert_eq!(
//         contract.get_cupcake_balance_for(user1).unwrap(),
//         U256::from(2)
//     );
//     assert_eq!(
//         contract.get_cupcake_balance_for(user2).unwrap(),
//         U256::from(1)
//     );
// }
