#![cfg_attr(not(any(feature = "export-abi", feature = "testing")), no_main)]

#[cfg(feature = "export-abi")]
fn main() {
    stylus_cupcake_example::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}
