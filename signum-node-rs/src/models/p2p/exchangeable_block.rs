use crate::models::p2p::ExchangeableTransaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeableBlock {
    version: u8,
    timestamp: String,
    previous_block: u64,
    total_amount_nqt: u64,
    total_fee_nqt: u64,
    total_fee_cashback_nqt: u64,
    total_fee_burnt_nqt: u64,
    payload_length: u32,
    payload_hash: String,
    generator_public_key: String,
    generation_signature: String,
    /// `previous_block_hash` is only valid in v1 blocks.
    previous_block_hash: Option<String>,
    block_signature: String,
    transactions: Vec<ExchangeableTransaction>,
    nonce: u64,
    base_target: u64,
    block_ats: String,
}
