use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct B1Transaction {
    #[serde(rename = "type")]
    pub transaction_type: u8,
    pub subtype: u8,
    pub timestamp: u64,
    pub deadline: u16,
    pub sender_public_key: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub recipient: Option<u64>, // TODO Why is this an Option? Are transaction recipients optional?
    #[serde(rename = "amountNQT")]
    pub amount_nqt: u64,
    #[serde(rename = "feeNQT")]
    pub fee_nqt: u64,
    //pub referenced_transaction_full_hash: String,
    #[serde(rename = "ecBlockHeight")]
    pub ec_block_height: u32,
    #[serde(rename = "ecBlockId")]
    #[serde_as(as = "DisplayFromStr")]
    pub ec_block_id: u64,
    #[serde(rename = "cashBackId")]
    #[serde_as(as = "DisplayFromStr")]
    pub cash_back_id: u64,
    pub signature: String,
    //pub attachment: Vec<B1TransactionAttachment>,
    pub version: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum B1TransactionAttachment {
    Message,
    EncryptedMessage,
    EncryptMessageToSelf,
    PublicKeyAnnouncement,
}
