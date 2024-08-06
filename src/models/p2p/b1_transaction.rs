use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct B1Transaction {
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub subtype: String,
    pub timestamp: String,
    pub deadline: u16,
    pub sender_public_key: String,
    pub recipient: Option<u64>,
    pub amount_nqt: u64,
    pub fee_nqt: u64,
    pub referenced_transaction_full_hash: String,
    pub ec_block_height: u32,
    pub ec_block_id: u64,
    pub cash_back_id: u64,
    pub signature: String,
    pub attachment: Vec<B1TransactionAttachment>,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum B1TransactionAttachment {
    Message,
    EncryptedMessage,
    EncryptMessageToSelf,
    PublicKeyAnnouncement,
}
