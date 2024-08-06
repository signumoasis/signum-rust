use crate::models::p2p::B1Transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct B1Block {
    version: u8,
    timestamp: String,
    previous_block: u64,
    #[serde(rename = "totalAmountNQT")]
    total_amount_nqt: u64,
    #[serde(rename = "totalFeeNQT")]
    total_fee_nqt: u64,
    #[serde(rename = "totalFeeCashbasckNQT")]
    total_fee_cashback_nqt: u64,
    #[serde(rename = "totalFeeBurntNQT")]
    total_fee_burnt_nqt: u64,
    payload_length: u32,
    payload_hash: String,
    generator_public_key: String,
    generation_signature: String,
    /// `previous_block_hash` is only valid in v1 blocks.
    previous_block_hash: Option<String>,
    block_signature: String,
    transactions: Vec<B1Transaction>,
    nonce: u64,
    base_target: u64,
    #[serde(rename = "blockATs")]
    block_ats: Option<String>,
}
