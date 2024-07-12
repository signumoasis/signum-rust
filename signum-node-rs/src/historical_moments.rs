//! This module contains all the important historical moments that the chain has
//! experienced. These are hard forking changes. They are defaulted but can be overridden
//! by settings in the configuration file.
use std::sync::OnceLock;

use crate::configuration::HistoricalMoments;

/// The height at which the genesis block was created. Always 0.
pub static GENESIS: OnceLock<u32> = OnceLock::new();
/// The height at which the reward recipient feature was enabled.
pub static REWARD_RECIPIENT_ENABLE: OnceLock<u32> = OnceLock::new();
/// The height at which the digital goods store was enabled.
pub static DIGITAL_GOODS_STORE_ENABLE: OnceLock<u32> = OnceLock::new();
/// The height at which the automated transactions feature was enabled.
pub static AUTOMATED_TRANSACTION_ENABLE: OnceLock<u32> = OnceLock::new();
/// The height at which the automated transactions feature was fixed the first time.
//TODO: Try to find a description of the fix
pub static AUTOMATED_TRANSACTION_FIX_1: OnceLock<u32> = OnceLock::new();
/// The height at which the automated transactions feature was fixed the second time.
//TODO: Try to find a description of the fix
pub static AUTOMATED_TRANSACTION_FIX_2: OnceLock<u32> = OnceLock::new();
/// The height at which the automated transactions feature was fixed the third time.
//TODO: Try to find a description of the fix
pub static AUTOMATED_TRANSACTION_FIX_3: OnceLock<u32> = OnceLock::new();
/// The height at which the pre-PoC2 format prepration was enabled.
pub static PRE_POC2: OnceLock<u32> = OnceLock::new();
/// The height at which the PoC2 format was fully enabled.
pub static POC2_ENABLE: OnceLock<u32> = OnceLock::new();
/// The height at which the sodium feature was enabled.
pub static SODIUM_ENABLE: OnceLock<u32> = OnceLock::new();
/// The height at which the project was renamed from BurstCoin to Signum.
pub static SIGNUM_NAME_CHANGE: OnceLock<u32> = OnceLock::new();
/// The height at which the PoC+ format was enabled.
pub static POC_PLUS_ENABLE: OnceLock<u32> = OnceLock::new();
/// The height at which the speedway feature was enabled.
pub static SPEEDWAY_ENABLE: OnceLock<u32> = OnceLock::new();
/// The height at which the smart token feature was enabled.
pub static SMART_TOKEN_ENABLE: OnceLock<u32> = OnceLock::new();
/// The height at which the smart fees feature was enabled.
pub static SMART_FEES_ENABLE: OnceLock<u32> = OnceLock::new();
/// The height at which the smart automated transactions feature was enabled.
pub static SMART_ATS_ENABLE: OnceLock<u32> = OnceLock::new();
/// The height at which the automated transactions feature was fixed the fourth time.
//TODO: Try to find a description of the fix
pub static AUTOMATED_TRANSACTION_FIX_4: OnceLock<u32> = OnceLock::new();
/// The height at whcih the distribution fix was enabled.
pub static DISTRIBUTION_FIX_ENABLE: OnceLock<u32> = OnceLock::new();
/// The height at which the first public key freeze was enacted.
pub static PK_FREEZE: OnceLock<u32> = OnceLock::new();
/// The height at which the second public key freeze was enacted.
pub static PK_FREEZE_2: OnceLock<u32> = OnceLock::new();
/// The height at which the smart alias feature was enabled.
pub static SMART_ALIAS_ENABLE: OnceLock<u32> = OnceLock::new();
/// The next fork height is always u32::MAX to ensure the flux values alwasy have a value.
pub static NEXT_FORK: OnceLock<u32> = OnceLock::new();

pub fn initialize_historical_moments(settings: HistoricalMoments) -> anyhow::Result<()> {
    GENESIS
        .set(0)
        .map_err(|e| anyhow::anyhow!("could not set 'GENESIS' with value {:?}", e))?;
    REWARD_RECIPIENT_ENABLE
        .set(settings.reward_recipient_enable.unwrap_or(6500))
        .map_err(|e| {
            anyhow::anyhow!("could not set 'REWARD_RECIPIENT_ENABLE' with value {:?}", e)
        })?;
    DIGITAL_GOODS_STORE_ENABLE
        .set(settings.digital_goods_store_enable.unwrap_or(11_800))
        .map_err(|e| {
            anyhow::anyhow!(
                "could not set 'DIGITAL_GOODS_STORE_ENABLE' with value {:?}",
                e
            )
        })?;
    AUTOMATED_TRANSACTION_ENABLE
        .set(settings.automated_transaction_enable.unwrap_or(49_200))
        .map_err(|e| {
            anyhow::anyhow!(
                "could not set 'AUTOMATED_TRANSACTION_ENABLE' with value {:?}",
                e
            )
        })?;
    AUTOMATED_TRANSACTION_FIX_1
        .set(settings.automated_transaction_fix_1.unwrap_or(67_000))
        .map_err(|e| {
            anyhow::anyhow!(
                "could not set 'AUTOMATED_TRANSACTION_FIX_1' with value {:?}",
                e
            )
        })?;
    AUTOMATED_TRANSACTION_FIX_2
        .set(settings.automated_transaction_fix_2.unwrap_or(92_000))
        .map_err(|e| {
            anyhow::anyhow!(
                "could not set 'AUTOMATED_TRANSACTION_FIX_2' with value {:?}",
                e
            )
        })?;
    AUTOMATED_TRANSACTION_FIX_3
        .set(settings.automated_transaction_fix_3.unwrap_or(255_000))
        .map_err(|e| {
            anyhow::anyhow!(
                "could not set 'AUTOMATED_TRANSACTION_FIX_3' with value {:?}",
                e
            )
        })?;
    PRE_POC2
        .set(settings.pre_poc2.unwrap_or(500_000))
        .map_err(|e| anyhow::anyhow!("could not set 'PRE_POC2' with value {:?}", e))?;
    POC2_ENABLE
        .set(settings.poc2_enable.unwrap_or(502_000))
        .map_err(|e| anyhow::anyhow!("could not set 'POC2_ENABLE' with value {:?}", e))?;
    SODIUM_ENABLE
        .set(settings.sodium_enable.unwrap_or(765_000))
        .map_err(|e| anyhow::anyhow!("could not set 'SODIUM_ENABLE' with value {:?}", e))?;
    SIGNUM_NAME_CHANGE
        .set(settings.signum_name_change.unwrap_or(875_000))
        .map_err(|e| anyhow::anyhow!("could not set 'SIGNUM_NAME_CHANGE' with value {:?}", e))?;
    POC_PLUS_ENABLE
        .set(settings.poc_plus_enable.unwrap_or(878_000))
        .map_err(|e| anyhow::anyhow!("could not set 'POC_PLUS_ENABLE' with value {:?}", e))?;
    SPEEDWAY_ENABLE
        .set(settings.speedway_enable.unwrap_or(941_100))
        .map_err(|e| anyhow::anyhow!("could not set 'SPEEDWAY_ENABLE' with value {:?}", e))?;
    SMART_TOKEN_ENABLE
        .set(settings.smart_token_enable.unwrap_or(1_029_000))
        .map_err(|e| anyhow::anyhow!("could not set 'SMART_TOKEN_ENABLE' with value {:?}", e))?;
    SMART_FEES_ENABLE
        .set(settings.smart_fees_enable.unwrap_or(1_029_000))
        .map_err(|e| anyhow::anyhow!("could not set 'SMART_FEES_ENABLE' with value {:?}", e))?;
    SMART_ATS_ENABLE
        .set(settings.smart_ats_enable.unwrap_or(1_029_000))
        .map_err(|e| anyhow::anyhow!("could not set 'SMART_ATS_ENABLE' with value {:?}", e))?;
    AUTOMATED_TRANSACTION_FIX_4
        .set(settings.automated_transaction_fix_4.unwrap_or(1_051_900))
        .map_err(|e| {
            anyhow::anyhow!(
                "could not set 'AUTOMATED_TRANSACTION_FIX_4' with value {:?}",
                e
            )
        })?;
    DISTRIBUTION_FIX_ENABLE
        .set(settings.distribution_fix_enable.unwrap_or(1_051_900))
        .map_err(|e| {
            anyhow::anyhow!("could not set 'DISTRIBUTION_FIX_ENABLE' with value {:?}", e)
        })?;
    PK_FREEZE
        .set(settings.pk_freeze.unwrap_or(1_099_400))
        .map_err(|e| anyhow::anyhow!("could not set 'PK_FREEZE' with value {:?}", e))?;
    PK_FREEZE_2
        .set(settings.pk_freeze_2.unwrap_or(1_150_000))
        .map_err(|e| anyhow::anyhow!("could not set 'PK_FREEZE_2' with value {:?}", e))?;
    SMART_ALIAS_ENABLE
        .set(settings.smart_alias_enable.unwrap_or(1_150_000))
        .map_err(|e| anyhow::anyhow!("could not set 'SMART_ALIAS_ENABLE' with value {:?}", e))?;
    NEXT_FORK
        .set(settings.next_fork.unwrap_or(u32::MAX))
        .map_err(|e| anyhow::anyhow!("could not set 'NEXT_FORK' with value {:?}", e))?;

    Ok(())
}
