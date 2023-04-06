pub use crate::traits::staking::*;
use openbrush::{
    storage::Mapping,
    traits::{
        Storage, Timestamp
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[openbrush::upgradeable_storage(STORAGE_KEY)]
#[derive(Default, Debug)]
pub struct Data {
    pub staked: Mapping<AccountId, (Balance, Timestamp)>,
    pub _reserved: Option<()>,
}

impl<T> Staking for T
where
    T: Storage<Data>,
    T: PSP22,
    T: psp22::Internal,
{
    fn stake(&mut self, amount: Balance) -> Result<(), StakingErr> {
        
        // get the AccountId of this caller
        let caller = Self::env().caller();
        
        // get the staking data which will return a &Option<(u128, u64)
        let staked = &self.data::<Data>().staked.get(&caller);

        // users must enter an amount greater than zero
        if amount == 0 {
            return Err(StakingErr::AmountMustBeAboveZero)
        }

        // if the caller has some tokens already staked, accumulate the amount
        if let Some(staking_data) = staked {
            // accumulate stake using Internal trait
            self._accummulate(&staking_data)?;
        } else {
            // otherwise, insert the amount to the staking data 
            let _ = &self
                .data::<Data>()
                .staked
                .insert(&caller, &(amount, Self::env().block_timestamp()));
        }
        
        // transfer the amount to the contract's account ID
        self.transfer_from(caller, Self::env().account_id(), amount, Vec::default())?;

        Ok(())
    }

    fn unstake(&mut self, amount: Balance) -> Result<(), StakingErr> {
        
        // get the AccountId of this caller
        let caller = Self::env().caller();
        
        // get the staking data which will return a &Option<(u128, u64)
        let staked = &self.data::<Data>().staked.get(&caller);

        // users must enter an amount greater than zero
        if amount == 0 {
            return Err(StakingErr::AmountMustBeAboveZero)
        }

        if let Some(staking_data) = staked {
            
            // accumulate stake using Internal trait
            let new_amount = self._accummulate(&staking_data)?;
            
            // if user input is equal to or more than their current stake, withdraw max
            if amount >= new_amount {
                self.data().staked.remove(&caller);

                // transfer the amount wanted to be unstaked back to the caller
                self.transfer_from(Self::env().account_id(), caller, new_amount, Vec::default())?;
            } else {

                // update the staked amount 
                self.data()
                    .staked
                    .insert(&caller, &(new_amount - amount, Self::env().block_timestamp()));
                
                // transfer the amount wanted to be unstaked back to the caller
                self.transfer_from(Self::env().account_id(), caller, amount, Vec::default())?;
            }
        } else {
            // no stake
            return Err(StakingErr::NothingToWithdraw)
        }

        Ok(())
    }

    fn voting_power(&self, account: AccountId) -> u128 {
        
        // get the amount the account ID has staked 
        let staked = &self.data::<Data>().staked.get(&account);

        // use Internal trait to get voting power
        if let Some(staking_data) = staked {
            return self._get_accumulated_amount(&staking_data)
        } else {
            0
        }
    }
}