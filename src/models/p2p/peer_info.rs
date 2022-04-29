use serde::Deserialize;

use super::PeerAddress;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerInfo {
    pub announced_address: Option<PeerAddress>,
    pub application: String,
    pub version: String,
    pub platform: Option<String>,
    pub share_address: bool,
}
