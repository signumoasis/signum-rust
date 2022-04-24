use std::collections::HashMap;

use rand::prelude::{IteratorRandom, SliceRandom};
use serde::Deserialize;
use tokio::sync::{mpsc, oneshot};

/// The PeerService discovers, manages, and blacklists peers.
/// It also returns peers to be used by other services.
pub struct PeerService {
    receiver: mpsc::Receiver<PeerMessage>,
    peers_cache: HashMap<PeerAddress, Peer>,
    blacklisted_peer_cache: HashMap<PeerAddress, Peer>,
}
impl PeerService {
    #[tracing::instrument(name = "PeerServiceq.new()", skip(receiver))]
    pub fn new(receiver: mpsc::Receiver<PeerMessage>) -> Self {
        Self {
            receiver,
            peers_cache: HashMap::<PeerAddress, Peer>::new(),
            blacklisted_peer_cache: HashMap::<PeerAddress, Peer>::new(),
        }
    }

    #[tracing::instrument(name = "PeerService.handle_message()", skip(self))]
    fn handle_message(&mut self, msg: PeerMessage) {
        match msg {
            PeerMessage::GetRandomPeer { respond_to } => {
                //TODO: Consider randomly selecting from the database?
                match self.peers_cache.keys().choose(&mut rand::thread_rng()) {
                    Some(key) => {
                        let _ = respond_to.send(self.peers_cache.get(key).cloned());
                    }
                    None => {
                        let _ = respond_to.send(None);
                    }
                }
            }
            PeerMessage::GetPeer {
                respond_to,
                peer_address,
            } => {
                let peer = self.peers_cache.get(&peer_address);
                let _ = respond_to.send(peer.cloned());
            }
            PeerMessage::BlacklistPeer { respond_to } => todo!(),
        }
    }
}

/// A handle to interact with the [`PeerService`].
#[derive(Clone)]
pub struct PeerServiceHandle {
    sender: mpsc::Sender<PeerMessage>,
}
impl PeerServiceHandle {
    #[tracing::instrument(name = "PeerServiceHandle.new()")]
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = PeerService::new(receiver);
        tokio::spawn(run_peer_service(actor));

        Self { sender }
    }

    /// Requests a random `Option<[Peer]>` from the [`PeerService`]. Returns None if
    /// no peer can be found.
    #[tracing::instrument(name = "PeerServiceHandle.get_random_peer()", skip(self))]
    pub async fn get_random_peer(&self) -> Option<Peer> {
        let (send, recv) = oneshot::channel();
        let msg = PeerMessage::GetRandomPeer { respond_to: send };

        let _ = self.sender.send(msg).await;
        recv.await.expect("PeerService has been killed")
    }

    /// Requests a specific `Option<[Peer]>` by `[PeerAddress]` from the [`PeerService`]. Returns None if
    /// no peer can be found.
    #[tracing::instrument(name = "PeerServiceHandle.get_peer_by_address()", skip(self))]
    pub async fn get_peer_by_address(&self, address: PeerAddress) -> Option<Peer> {
        todo!()
    }
}

impl Default for PeerServiceHandle {
    fn default() -> Self {
        Self::new()
    }
}

#[tracing::instrument(name = "run_peer_service", skip(service))]
async fn run_peer_service(mut service: PeerService) {
    //TODO: spawn tasks for peer discover etc
    while let Some(msg) = service.receiver.recv().await {
        service.handle_message(msg);
    }
}

#[derive(Debug)]
pub enum PeerMessage {
    GetRandomPeer {
        respond_to: oneshot::Sender<Option<Peer>>,
    },
    GetPeer {
        respond_to: oneshot::Sender<Option<Peer>>,
        peer_address: PeerAddress,
    },
    BlacklistPeer {
        respond_to: oneshot::Sender<anyhow::Result<()>>,
    },
}

/// Necessary because of poor json formatting decisions in the source data.
#[derive(Debug, Deserialize)]
pub struct PeerContainer {
    pub peers: Vec<PeerAddress>,
}

// TODO: This peer object needs to be separated into two
// One for the json deserialization and one for internal use
// because the json doesn't include the actual IP and port
// but this app needs to serialize and deserialize them
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
    /// The peer's actual IP address.
    #[serde(skip)]
    ip_address: String,
    #[serde(skip)]
    port: u32,
    /// The peer's self-advertised address. May not be present.
    announced_address: Option<PeerAddress>,
    application: String,
    version: String,
    platform: Option<String>,
    share_address: bool,
}
impl Peer {
    fn get_peers() {}
    fn get_info() {}
    fn get_milestone_block_ids() {}
    fn get_next_block_ids() {}
    fn get_unconfirmed_transactions() {}
    fn request_add_peers() {}
    fn request_process_block() {}
    fn request_process_transactions() {}
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
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
