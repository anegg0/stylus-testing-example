//!
//! Stylus Cupcake Example
//!
//! The contract is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!
//! Note: this code is a template-only and has not been audited.
//!

#![cfg_attr(not(any(feature = "export-abi")), no_main)]
extern crate alloc;

use alloy_primitives::Uint;
use stylus_sdk::alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*;
use stylus_sdk::{block, console};
sol_storage! {
    #[entrypoint]
    pub struct VendingMachine {
        mapping(address => uint256) cupcake_balances;
        mapping(address => uint256) cupcake_distribution_times;
    }
}

// Declare that `VendingMachine` is a contract with the following external methods.
#[public]
impl VendingMachine {
    pub fn give_cupcake_to(&mut self, user_address: Address) -> bool {
        let last_distribution = self.cupcake_distribution_times.get(user_address);
        let five_seconds_from_last_distribution = last_distribution + U256::from(5);

        let current_time = block::timestamp();
        let user_can_receive_cupcake =
            five_seconds_from_last_distribution <= Uint::<256, 4>::from(current_time);

        if user_can_receive_cupcake {
            let mut balance_accessor = self.cupcake_balances.setter(user_address);
            let balance = balance_accessor.get() + U256::from(1);
            balance_accessor.set(balance);

            let mut time_accessor = self.cupcake_distribution_times.setter(user_address);
            let new_distribution_time = block::timestamp();
            time_accessor.set(Uint::<256, 4>::from(new_distribution_time));
            return true;
        } else {
            console!(
                "HTTP 429: Too Many Cupcakes (you must wait at least 5 seconds between cupcakes)"
            );
            return false;
        }
    }

    pub fn get_cupcake_balance_for(&self, user_address: Address) -> Uint<256, 4> {
        return self.cupcake_balances.get(user_address);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_give_cupcake() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut vending_machine = VendingMachine::from(&vm);

        // Set current timestamp
        vm.set_block_timestamp(100);

        // Get a user address
        let user = Address::repeat_byte(1);

        // Initially, user should have 0 cupcakes
        assert_eq!(vending_machine.get_cupcake_balance_for(user), U256::ZERO);

        // Give a cupcake - should succeed
        assert!(vending_machine.give_cupcake_to(user));

        // User should now have 1 cupcake
        assert_eq!(vending_machine.get_cupcake_balance_for(user), U256::from(1));

        // Try to get another cupcake immediately - should fail
        assert!(!vending_machine.give_cupcake_to(user));

        // User should still have 1 cupcake
        assert_eq!(vending_machine.get_cupcake_balance_for(user), U256::from(1));

        // Advance time by 6 seconds
        vm.set_block_timestamp(106);

        // Try again - should succeed
        assert!(vending_machine.give_cupcake_to(user));

        // User should now have 2 cupcakes
        assert_eq!(vending_machine.get_cupcake_balance_for(user), U256::from(2));
    }

    #[test]
    fn test_multiple_users() {
        let vm = TestVM::default();
        let mut vending_machine = VendingMachine::from(&vm);

        // Set current timestamp
        vm.set_block_timestamp(100);

        // Define two different users
        let user1 = Address::repeat_byte(1);
        let user2 = Address::repeat_byte(2);

        // Give a cupcake to user1
        assert!(vending_machine.give_cupcake_to(user1));

        // User1 should have 1 cupcake, user2 should have 0
        assert_eq!(
            vending_machine.get_cupcake_balance_for(user1),
            U256::from(1)
        );
        assert_eq!(vending_machine.get_cupcake_balance_for(user2), U256::ZERO);

        // Give a cupcake to user2
        assert!(vending_machine.give_cupcake_to(user2));

        // Both users should have 1 cupcake
        assert_eq!(
            vending_machine.get_cupcake_balance_for(user1),
            U256::from(1)
        );
        assert_eq!(
            vending_machine.get_cupcake_balance_for(user2),
            U256::from(1)
        );

        // Advance time
        vm.set_block_timestamp(106);

        // Give another cupcake to user1
        assert!(vending_machine.give_cupcake_to(user1));

        // User1 should have 2 cupcakes, user2 still has 1
        assert_eq!(
            vending_machine.get_cupcake_balance_for(user1),
            U256::from(2)
        );
        assert_eq!(
            vending_machine.get_cupcake_balance_for(user2),
            U256::from(1)
        );
    }
}
