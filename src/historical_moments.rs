//! This module contains all the important historical moments that the chain has
//! experienced. These are hard forking changes. They are defaulted but can be overridden
//! by settings in the configuration file.

use std::marker::PhantomData;

pub trait HistoricalMomentsState {}

pub struct Building;
impl HistoricalMomentsState for Building {}

pub struct Active;
impl HistoricalMomentsState for Active {}

pub struct HistoricalMoments<S: HistoricalMomentsState> {
    _phantom: PhantomData<S>,
}

impl HistoricalMoments<Building> {
    pub fn new() -> HistoricalMoments<Building> {
        HistoricalMoments::<Building> {
            _phantom: PhantomData,
        }
    }
    pub fn build(&self) -> HistoricalMoments<Active> {
        HistoricalMoments::<Active> {
            _phantom: PhantomData,
        }
    }
}

impl Default for HistoricalMoments<Building> {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoricalMoments<Active> {
    /// The height at which the genesis block was created. Always 0.
    pub fn genesis(&self) -> u32 {
        todo!()
    }

    /// The height at which the reward recipient feature was enabled.
    pub fn reward_recipient_enable(&self) -> u32 {
        todo!()
    }

    /// The height at which the digital goods store was enabled.
    pub fn digital_goods_store_enable(&self) -> u32 {
        todo!()
    }

    /// The height at which the automated transactions feature was enabled.
    pub fn automated_transaction_enable(&self) -> u32 {
        todo!()
    }

    /// The height at which the automated transactions feature was fixed the first time.
    //TODO: Try to find a description of the fix
    pub fn automated_transaction_fix_1(&self) -> u32 {
        todo!()
    }

    /// The height at which the automated transactions feature was fixed the second time.
    //TODO: Try to find a description of the fix
    pub fn automated_transaction_fix_2(&self) -> u32 {
        todo!()
    }

    /// The height at which the automated transactions feature was fixed the third time.
    //TODO: Try to find a description of the fix
    pub fn automated_transaction_fix_3(&self) -> u32 {
        todo!()
    }

    /// The height at which the pre-PoC2 format prepration was enabled.
    pub fn pre_poc2(&self) -> u32 {
        todo!()
    }

    /// The height at which the PoC2 format was fully enabled.
    pub fn poc2_enable(&self) -> u32 {
        todo!()
    }

    /// The height at which the sodium feature was enabled.
    pub fn sodium_enable(&self) -> u32 {
        todo!()
    }

    /// The height at which the project was renamed from BurstCoin to Signum.
    pub fn signum_name_change(&self) -> u32 {
        todo!()
    }

    /// The height at which the PoC+ format was enabled.
    pub fn poc_plus_enable(&self) -> u32 {
        todo!()
    }

    /// The height at which the speedway feature was enabled.
    pub fn speedway_enable(&self) -> u32 {
        todo!()
    }

    /// The height at which the smart token feature was enabled.
    pub fn smart_token_enable(&self) -> u32 {
        todo!()
    }

    /// The height at which the smart fees feature was enabled.
    pub fn smart_fees_enable(&self) -> u32 {
        todo!()
    }

    /// The height at which the smart automated transactions feature was enabled.
    pub fn smart_ats_enable(&self) -> u32 {
        todo!()
    }

    /// The height at which the automated transactions feature was fixed the fourth time.
    //TODO: Try to find a description of the fix
    pub fn automated_transaction_fix_4(&self) -> u32 {
        todo!()
    }

    /// The height at whcih the distribution fix was enabled.
    pub fn distribution_fix_enable(&self) -> u32 {
        todo!()
    }

    /// The height at which the first public key freeze was enacted.
    pub fn pk_freeze(&self) -> u32 {
        todo!()
    }

    /// The height at which the second public key freeze was enacted.
    pub fn pk_freeze_2(&self) -> u32 {
        todo!()
    }

    /// The height at which the smart alias feature was enabled.
    pub fn smart_alias_enable(&self) -> u32 {
        todo!()
    }

    /// The next fork height is always u32::MAX to ensure the flux values alwasy have a value.
    pub fn next_fork(&self) -> u32 {
        todo!()
    }
}
