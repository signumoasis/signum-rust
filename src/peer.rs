use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::models::p2p::{BlockId, PeerAddress, PeerInfo, Transaction};

#[derive(Debug)]
pub enum PeerMessage {
    CallGetPeers {
        respond_to: mpsc::Sender<Vec<PeerAddress>>,
    },
    CallGetInfo {
        respond_to: mpsc::Sender<PeerInfo>,
    },
    CallGetMilestoneBlockIds {
        respond_to: mpsc::Sender<Vec<BlockId>>,
    },
    CallGetNextBlockIds {
        respond_to: mpsc::Sender<Vec<BlockId>>,
    },
    CallUnconfirmedTransactions {
        respond_to: mpsc::Sender<Vec<Transaction>>,
    },
    CallAddPeers {
        respond_to: mpsc::Sender<anyhow::Result<()>>,
    },
    CallProcessBlock {
        respond_to: mpsc::Sender<anyhow::Result<()>>,
    },
    CallProcessTransactions {
        respond_to: mpsc::Sender<anyhow::Result<()>>,
    },
}

#[derive(Debug)]
struct Peer {
    receiver: mpsc::Receiver<PeerMessage>,
}
impl Peer {
    #[tracing::instrument(name = "Peer.new()")]
    pub fn new(receiver: mpsc::Receiver<PeerMessage>) -> Self {
        Self { receiver }
    }

    #[tracing::instrument(name = "Peer.handle_message()", skip(self))]
    fn handle_message(&mut self, msg: PeerMessage) {
        todo!();
    }
}

#[derive(Debug)]
pub struct PeerHandle {
    sender: mpsc::Sender<PeerMessage>,
}
impl PeerHandle {
    #[tracing::instrument(name = "PeerHandle.new()")]
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = Peer::new(receiver);
        tokio::spawn(run_peer_actor(actor));

        Self { sender }
    }

    #[tracing::instrument(name = "PeerHandle.get_peers()")]
    pub async fn call_get_peers() -> Vec<PeerAddress> {
        todo!();
    }

    #[tracing::instrument(name = "PeerHandle.get_info()")]
    pub async fn call_get_info() -> PeerInfo {
        todo!();
    }

    #[tracing::instrument(name = "PeerHandle.get_milestone_block_ids()")]
    pub async fn call_get_milestone_block_ids() -> Vec<BlockId> {
        todo!();
    }

    #[tracing::instrument(name = "PeerHandle.get_next_block_ids()")]
    pub async fn call_get_next_block_ids() -> Vec<BlockId> {
        todo!();
    }

    #[tracing::instrument(name = "PeerHandle.unconfirmed_transactions()")]
    pub async fn call_get_unconfirmed_transactions() -> Vec<Transaction> {
        todo!();
    }

    #[tracing::instrument(name = "PeerHandle.add_peers()")]
    pub async fn call_add_peers() -> anyhow::Result<()> {
        todo!();
    }

    #[tracing::instrument(name = "PeerHandle.process_block()")]
    pub async fn call_process_block() -> anyhow::Result<()> {
        todo!();
    }

    #[tracing::instrument(name = "PeerHandle.new()")]
    pub async fn call_process_transactions() -> anyhow::Result<()> {
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
