use serde::Deserialize;
use tokio::sync::{mpsc, oneshot};

pub struct PeerService {
    receiver: mpsc::Receiver<PeerMessage>,
    peers_cache: Vec<Peer>,
}

pub enum PeerMessage {
    PlaceHolder,
}

/// Necessary because of poor json formatting decisions in the source data.
#[derive(Debug, Deserialize)]
pub struct PeerContainer {
    pub peers: Vec<PeerAddress>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
    announced_address: Option<PeerAddress>,
    application: String,
    version: String,
    platform: Option<String>,
    share_address: bool,
}

#[derive(Debug, Deserialize)]
// #[serde(try_from = "String")]
#[serde(transparent)]
pub struct PeerAddress(pub String);
// impl TryFrom<String> for PeerAddress {
//     type Error = anyhow::Error;

//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         if validate_peer_address(&value) {
//             Ok(Self(value))
//         } else {
//             Err(anyhow::anyhow!("Invalid peer address: {}", value))
//         }
//     }
// }

// fn validate_peer_address(value: &str) -> bool {

// }
