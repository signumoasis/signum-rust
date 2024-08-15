mod b1_peer;
mod oasis_peer;

pub use b1_peer::B1Peer;
pub use oasis_peer::OasisPeer;

use actix_web::ResponseError;
use anyhow::Result;
use num_bigint::BigUint;

use crate::models::{
    datastore::Datastore,
    p2p::{B1Block, PeerAddress, PeerInfo},
    Block,
};

// TODO: Move this to models or something
/// A downloaded set of blocks.
#[derive(Debug)]
pub struct DownloadResult {
    pub blocks: Vec<Block>,
    pub peer: PeerAddress,
    pub start_height: u64,
    pub number_of_blocks: u32,
}

#[derive(Debug)]
pub enum BlockSelect {
    BRS(B1Peer),
    OASIS(OasisPeer),
}

#[allow(async_fn_in_trait)]
pub trait GetPeerInfo {
    async fn get_peer_info(&self) -> Result<(PeerInfo, String), PeerCommunicationError>;
}

#[allow(async_fn_in_trait)]
pub trait BasicPeerClient: GetPeerInfo {
    fn address(&self) -> PeerAddress;
    async fn get_blocks_from_height(
        &self,
        height: u64,
        number_of_blocks: u32,
    ) -> Result<DownloadResult, PeerCommunicationError>;
    async fn get_peers(&self) -> Result<Vec<PeerAddress>, anyhow::Error>;
    async fn get_peer_cumulative_difficulty(&self) -> Result<BigUint>;
}

/// Requests peer information from the supplied PeerAddress. Updates the database
/// with the acquired information. Returns a [`anyhow::Result<()>`].
#[tracing::instrument(name = "Update Info Task", skip_all)]
pub async fn update_db_peer_info(database: Datastore, peer: impl BasicPeerClient) -> Result<()> {
    let peer_info = peer.get_peer_info().await;
    match peer_info {
        Ok(info) => {
            tracing::trace!("PeerInfo: {:?}", &info);

            let ip = info.1;
            let info = info.0;

            let _response = database.update_peer_info(peer.address(), ip, info).await?;
        }
        Err(PeerCommunicationError::ConnectionError(e)) => {
            tracing::warn!(
                "Connection error to peer {}. Blacklisting.",
                &peer.address(),
            );
            tracing::debug!(
                "Connection error for {}: Caused by:\n\t{:#?}",
                &peer.address(),
                e
            );
            database
                .increment_attempts_since_last_seen(peer.address())
                .await?;
            database.blacklist_peer(peer.address()).await?;
        }
        Err(PeerCommunicationError::ConnectionTimeout(e)) => {
            // TODO: Blacklist only after a certain number of attempts_since_last_seen
            // TODO: deblacklist on every 10th attempt since last seen to give it a chance again?
            // tracing::warn!("Connection to peer {} has timed out. Blacklisting.", &peer);
            tracing::debug!(
                "Connection timeout for {}. Caused by: \n\t{:#?}",
                &peer.address(),
                e
            );

            database
                .increment_attempts_since_last_seen(peer.address())
                .await?;
            // database.blacklist_peer(peer).await?;
        }
        Err(PeerCommunicationError::ContentDecodeError(e)) => {
            tracing::warn!(
                "Peer {} response could not be properly decoded. Blacklisting peer.",
                &peer.address(),
            );
            tracing::debug!(
                "Peer {} decoding error. Caused by:\n\t{:#?}",
                &peer.address(),
                e
            );
            database.blacklist_peer(peer.address()).await?;
        }
        Err(PeerCommunicationError::UnexpectedError(e)) => {
            tracing::error!(
                "Problem getting peer info for {}. Caused by:\n\t{:#?}",
                &peer.address(),
                e
            );

            database
                .increment_attempts_since_last_seen(peer.address())
                .await?;
        }
    }

    Ok(())
}

#[derive(thiserror::Error)]
pub enum PeerCommunicationError {
    #[error("Missing announced address: {0}")]
    ContentDecodeError(#[source] reqwest::Error),
    #[error("Connection error {0}")]
    ConnectionError(#[source] reqwest::Error),
    #[error("Connection timeout {0}")]
    ConnectionTimeout(#[source] reqwest::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for PeerCommunicationError {}

impl std::fmt::Debug for PeerCommunicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::error_chain_fmt(self, f)
    }
}

/// Blacklist a client for minutes * blacklist_count, for a maximum of 24 hours.
/// blacklist_count increments by 1 each time a node is blacklisted, so it will
/// be ignored for longer and longer, up to 24 hours before retry.
pub async fn blacklist_peer(database: Datastore, peer: PeerAddress) -> Result<()> {
    let _response = database.blacklist_peer(peer).await?;
    Ok(())
}

/// De-blacklist a node. This should happen anytime this node queries it and receives
/// a correct response, or if it talks to this node with a correct introduction.
pub async fn deblacklist_peer(database: Datastore, peer: PeerAddress) -> Result<()> {
    let _response = database.deblacklist_peer(peer);
    Ok(())
}
