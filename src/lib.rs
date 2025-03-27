//!
//! Stylus Cupcake Example
//!
//! The contract is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!
//! Note: this code is a template-only and has not been audited.
//!

// Allow `cargo stylus export-abi` to generate a main function if the "export-abi" feature is enabled.
#![cfg_attr(not(any(feature = "export-abi", feature = "testing")), no_main)]
extern crate alloc;

use alloy_primitives::Uint;
// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::alloy_primitives::{U256, Address};
use stylus_sdk::prelude::*;
use stylus_sdk::{block, console};

// Define persistent storage using the Solidity ABI.
// `VendingMachine` will be the entrypoint for the contract.
sol_storage! {
    #[entrypoint]
    pub struct VendingMachine {
        // Mapping from user addresses to their cupcake balances.
        mapping(address => uint256) cupcake_balances;
        // Mapping from user addresses to the last time they received a cupcake.
        mapping(address => uint256) cupcake_distribution_times;
    }
}

// Declare that `VendingMachine` is a contract with the following external methods.
#[public]
impl VendingMachine {
    // Give a cupcake to the specified user if they are eligible (i.e., if at least 5 seconds have passed since their last cupcake).
    pub fn give_cupcake_to(&mut self, user_address: Address) -> bool {
        // Get the last distribution time for the user.
        let last_distribution = self.cupcake_distribution_times.get(user_address);
        // Calculate the earliest next time the user can receive a cupcake.
        let five_seconds_from_last_distribution = last_distribution + U256::from(5);

        // Get the current block timestamp.
        let current_time = block::timestamp();
        // Check if the user can receive a cupcake.
        let user_can_receive_cupcake =
            five_seconds_from_last_distribution <= Uint::<256, 4>::from(current_time);

        if user_can_receive_cupcake {
            // Increment the user's cupcake balance.
            let mut balance_accessor = self.cupcake_balances.setter(user_address);
            let balance = balance_accessor.get() + U256::from(1);
            balance_accessor.set(balance);

            // Update the distribution time to the current time.
            let mut time_accessor = self.cupcake_distribution_times.setter(user_address);
            let new_distribution_time = block::timestamp();
            time_accessor.set(Uint::<256, 4>::from(new_distribution_time));
            return true;
        } else {
            // User must wait before receiving another cupcake.
            console!(
                "HTTP 429: Too Many Cupcakes (you must wait at least 5 seconds between cupcakes)"
            );
            return false;
        }
    }

    // Get the cupcake balance for the specified user.
    pub fn get_cupcake_balance_for(&self, user_address: Address) -> Uint<256, 4> {
        // Return the user's cupcake balance from storage.
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
