use anyhow::Result;
use tokio::sync::{mpsc, oneshot};

use crate::models::{
    datastore::PeerState,
    p2p::{BlockId, ExchangeableBlock, PeerAddress, PeerInfo, Transaction},
};

/// Messages to send a request to the remote node this `[Peer]` represents.
#[derive(Debug)]
pub enum RemotePeerMessage {
    GetPeers {
        respond_to: oneshot::Sender<Vec<PeerAddress>>,
    },
    GetInfo {
        respond_to: oneshot::Sender<PeerInfo>,
    },
    GetMilestoneBlockIds {
        respond_to: oneshot::Sender<Vec<BlockId>>,
    },
    GetNextBlockIds {
        respond_to: oneshot::Sender<Vec<BlockId>>,
    },
    UnconfirmedTransactions {
        respond_to: oneshot::Sender<Vec<Transaction>>,
    },
    AddPeers {
        respond_to: oneshot::Sender<Result<()>>,
    },
    ProcessBlock {
        respond_to: oneshot::Sender<Result<()>>,
    },
    ProcessTransactions {
        respond_to: oneshot::Sender<Result<()>>,
    },
}

#[derive(Debug)]
pub enum PeerMessage {
    SetPeerInfo {
        respond_to: oneshot::Sender<Result<()>>,
    },
    GetPeerInfo {
        respond_to: oneshot::Sender<Option<PeerInfo>>,
    },
}

#[derive(Debug)]
struct Peer {
    receiver: mpsc::Receiver<RemotePeerMessage>,
    state: PeerState,
}
impl Peer {
    #[tracing::instrument(name = "Peer.new()")]
    pub fn new(receiver: mpsc::Receiver<RemotePeerMessage>, address: PeerAddress) -> Self {
        Self {
            receiver,
            state: PeerState {
                address,
                ..Default::default()
            },
        }
    }

    #[tracing::instrument(name = "Peer.handle_message()", skip(self))]
    fn handle_message(&mut self, msg: RemotePeerMessage) {
        todo!();
    }
}

#[derive(Clone, Debug)]
pub struct PeerHandle {
    sender: mpsc::Sender<RemotePeerMessage>,
}
impl PeerHandle {
    #[tracing::instrument(name = "PeerHandle.new()")]
    pub fn new(address: PeerAddress) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = Peer::new(receiver, address);
        tokio::spawn(run_peer_actor(actor));

        Self { sender }
    }

    /// Instructs the `[Peer]` to contact the remote node and
    /// get its list of peers.
    #[tracing::instrument(name = "PeerHandle.get_peers()")]
    pub async fn call_get_peers(&self) -> Result<Vec<PeerAddress>> {
        todo!();
    }

    /// Instructs the `[Peer]` to contact the remote node and
    /// get its server info. This will also push own node's
    /// info to the remote node.
    #[tracing::instrument(name = "PeerHandle.get_info()")]
    pub async fn call_get_info(&self) -> Result<PeerInfo> {
        todo!();
    }

    /// Instructs the `[Peer]` to contact the remote node and
    /// request a list of Milestone Block IDs.
    #[tracing::instrument(name = "PeerHandle.get_milestone_block_ids()")]
    pub async fn call_get_milestone_block_ids(&self) -> Result<Vec<BlockId>> {
        todo!();
    }

    /// Instructs the `[Peer]` to contact the remote node and
    /// request a list of the next Block IDs.
    #[tracing::instrument(name = "PeerHandle.get_next_block_ids()")]
    pub async fn call_get_next_block_ids(&self) -> Result<Vec<BlockId>> {
        todo!();
    }

    /// Instructs the `[Peer]` to contact the remote node and
    /// request all unconfirmed transactions.
    #[tracing::instrument(name = "PeerHandle.unconfirmed_transactions()")]
    pub async fn call_get_unconfirmed_transactions(&self) -> Result<Vec<Transaction>> {
        todo!();
    }

    /// Instructs the `[Peer]` to contact the remote node and
    /// request that it add a supplied peer.
    #[tracing::instrument(name = "PeerHandle.add_peers()")]
    pub async fn call_add_peers(&self, peer: PeerInfo) -> Result<()> {
        todo!();
    }

    /// Instructs the `[Peer]` to contact the remote node and
    /// request that it process a supplied block.
    #[tracing::instrument(name = "PeerHandle.process_block()")]
    pub async fn call_process_block(&self, block: ExchangeableBlock) -> Result<()> {
        todo!();
    }

    /// Instructs the `[Peer]` to contact the remote node and
    /// request that it process a supplied list of transactions.
    #[tracing::instrument(name = "PeerHandle.new()")]
    pub async fn call_process_transactions(&self, transactions: Vec<Transaction>) -> Result<()> {
        todo!();
    }

    pub async fn set_peer_info(info: PeerInfo) {
        todo!();
    }

    pub async fn get_peer_info() -> Option<PeerInfo> {
        todo!();
    }
}

async fn run_peer_actor(mut actor: Peer) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

// async fn get_request_p2p_client(
//     address: PeerAddress,
// ) -> Result<(reqwest::RequestBuilder, HashMap<String, String>), anyhow::Error> {
//     let ip = match address {
//         Some(announced_address) => {
//             match tokio::net::lookup_host(announced_address.0.to_string()).await {
//                 Ok(_) => announced_address.0.clone(),
//                 Err(e) => {
//                     return Err(anyhow::anyhow!(
//                         "error getting socket address from announced address: {}",
//                         e
//                     ));
//                 }
//             }
//         }
//         None => match tokio::net::lookup_host(&self.ip_address).await {
//             Ok(ips) => ips.collect::<Vec<_>>()[0].to_string(),
//             Err(e) => {
//                 return Err(anyhow::anyhow!("ip address could not be turned into socket address and no announced address present: {}",e));
//             }
//         },
//     };

//     let url = format!("http://{}", ip);
//     tracing::debug!("Signum P2P Url Requested: {}", &url);
//     let builder = reqwest::Client::new()
//         .post(url)
//         .header("User-Agent", USER_AGENT);
//     let mut body = HashMap::<String, String>::new();
//     body.insert("protocol".into(), "B1".into());
//     Ok((builder, body))
// }
