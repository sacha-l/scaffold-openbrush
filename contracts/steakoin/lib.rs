#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod steakoin {
    
    // Imports from openbrush
    use openbrush::{
        contracts::psp22::{
            Data as PSP22Data,
            *,
        },
        traits::Storage,
    };

    // Defines storage for your contract
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Steakoin {
    	#[storage_field]
		psp22: psp22::Data,
    }
    
    // Section contains default implementation without any modifications
	impl PSP22 for Steakoin {}
     
    impl Steakoin {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut _instance = Self::default();
			_instance._mint_to(_instance.env().caller(), initial_supply).expect("Should mint"); 
			_instance
        }
    }
}