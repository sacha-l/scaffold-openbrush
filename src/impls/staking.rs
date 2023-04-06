pub use crate::traits::staking::*;
use openbrush::{
    traits::{
        Storage,
    },
};

impl<T> Staking for T
where
    T: Storage<Data>,
    T: PSP22,
    T: psp22::Internal,
{
// Staking trait impl block goes here
}