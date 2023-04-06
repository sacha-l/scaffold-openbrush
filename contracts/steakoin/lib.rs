#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {

    // imports from openbrush
    use openbrush::{
        contracts::psp22::*,
        traits::Storage,
    };

    // The storage struct of our Staking contract
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Staking {
        /// Holds all PSP22 data as per [the PSP22 standard](https://github.com/w3f/PSPs/blob/master/PSPs/psp-22.md)
        #[storage_field]
        psp22: psp22::Data,
    }

    // Section contains default implementation without any modifications
    impl PSP22 for Staking {}

    impl Staking {
        #[ink(constructor)]
        /// Mint a fixed supply of 42_000_000 Staking tokens.
        ///
        /// Tokens are issued to the account instantiating this contract.
        pub fn new() -> Self {
            let mut _instance = Self::default();
            _instance
                ._mint_to(_instance.env().caller(), 42_000_000 * 10u128.pow(18))
                .expect("Should mint");
            _instance
        }
    }
}
