use serde::{Deserialize, Serialize};

use crate::models::p2p::PeerAddress;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PeerState {
    address: PeerAddress,
    blacklist_timestamp: Option<u64>, // If None, not blacklisted, else, time blacklist was issued
    brs_version: Option<String>,
    last_contact: Option<u64>, // unix timestamp or perhaps timestamp from signum epoch
    total_bytes_downloaded_lifetime: u64,
    total_bytes_uploaded_lifetime: u64,
}
