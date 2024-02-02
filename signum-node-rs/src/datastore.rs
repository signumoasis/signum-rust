use tokio::sync::oneshot;

use crate::models::{
    datastore::{Block, PeerState},
    p2p::{BlockId, PeerAddress},
};

pub enum DatastoreMessage {
    GetBlock {
        respond_to: oneshot::Sender<Block>,
        block_id: BlockId,
    },
    PutBlock {
        respond_to: oneshot::Sender<Block>,
        block: Block,
    },
    GetAllPeers {
        respond_to: oneshot::Sender<Vec<PeerState>>,
    },
    GetPeer {
        respond_to: oneshot::Sender<PeerState>,
        peer_address: PeerAddress,
    },
    PutPeer {
        respond_to: oneshot::Sender<anyhow::Result<()>>,
        peer_state: PeerState,
    },
}
