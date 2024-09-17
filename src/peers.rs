mod b1_peer;
mod oasis_peer;

use std::{str::FromStr, time::Duration};

pub use b1_peer::B1Peer;
pub use oasis_peer::OasisPeer;

use actix_web::ResponseError;
use anyhow::{Context, Result};
use num_bigint::BigUint;
use serde_json::json;

use crate::models::{
    datastore::Datastore,
    p2p::{PeerAddress, PeerInfo},
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
pub trait BasicPeerClient {
    fn address(&self) -> PeerAddress;
    async fn get_blocks_from_height(
        &self,
        height: u64,
        number_of_blocks: u32,
    ) -> Result<DownloadResult, PeerCommunicationError>;
    async fn get_peers(&self) -> Result<Vec<PeerAddress>, anyhow::Error>;
    async fn get_peer_cumulative_difficulty(&self) -> Result<BigUint>;
    async fn get_peer_info(&self) -> Result<(PeerInfo, String), PeerCommunicationError>;
}

/// Makes an http request to the supplied peer address and parses the returned information
/// into a [`PeerInfo`].
///
/// Returns a tuple of ([`PeerInfo`], [`String`]) where the string is the resolved IP
/// address of the peer.
#[tracing::instrument]
async fn get_peer_info(peer: &PeerAddress) -> Result<(PeerInfo, String), PeerCommunicationError> {
    let thebody = json!({
        "protocol": "B1",
        "requestType": "getInfo",
        "announcedAddress": "nodomain.com",
        "application": "BRS",
        "version": "3.8.0",
        "platform": "signum-rs",
        "shareAddress": "false",
    });

    //let response = post_peer_request(&thebody, None).await;
    let client = reqwest::Client::new()
        .post(peer.to_url())
        .timeout(Duration::from_secs(2))
        .header("User-Agent", "BRS/3.8.2")
        .json(&thebody);

    let response = client.send().await;

    let response = match response {
        Ok(r) => Ok(r),
        Err(e) if e.is_connect() => Err(PeerCommunicationError::ConnectionError(e)),
        Err(e) if e.is_timeout() => Err(PeerCommunicationError::ConnectionTimeout(e)),
        Err(e) => Err(PeerCommunicationError::UnexpectedError(
            Err(e).context("could not get a response")?,
        )),
    }?;

    let peer_ip = response
        .remote_addr()
        .ok_or_else(|| anyhow::anyhow!("peer response did not have an IP address"))?
        .ip()
        .to_string();

    tracing::trace!("found ip address {} for PeerAddress {}", &peer_ip, peer);

    let mut peer_info = match response.json::<PeerInfo>().await {
        Ok(i) => Ok(i),
        Err(e) if e.is_decode() => Err(PeerCommunicationError::ContentDecodeError(e)),
        Err(e) => Err(PeerCommunicationError::UnexpectedError(
            Err(e).context("could not convert body to PeerInfo")?,
        )),
    }?;

    // Use the peer ip if there is no announced_address
    if peer_info.announced_address.is_none() {
        peer_info.announced_address = Some(PeerAddress::from_str(&peer_ip)?);
    }

    Ok((peer_info, peer_ip))
}

/// Requests peer information from the supplied PeerAddress. Updates the database
/// with the acquired information. Returns a [`anyhow::Result<()>`].
#[tracing::instrument(name = "Update Info Task", skip_all)]
pub async fn update_db_peer_info(database: Datastore, peer: PeerAddress) -> Result<()> {
    let peer_info = get_peer_info(&peer).await;
    match peer_info {
        Ok(info) => {
            tracing::trace!("PeerInfo: {:?}", &info);

            let ip = info.1;
            let info = info.0;

            let _response = database.update_peer_info(peer, ip, info).await?;
        }
        Err(PeerCommunicationError::ConnectionError(e)) => {
            tracing::warn!("Connection error to peer {}. Blacklisting.", &peer,);
            tracing::debug!("Connection error for {}: Caused by:\n\t{:#?}", &peer, e);
            database.increment_attempts_since_last_seen(&peer).await?;
            database.blacklist_peer(&peer).await?;
        }
        Err(PeerCommunicationError::ConnectionTimeout(e)) => {
            // TODO: Blacklist only after a certain number of attempts_since_last_seen
            // TODO: deblacklist on every 10th attempt since last seen to give it a chance again?
            // tracing::warn!("Connection to peer {} has timed out. Blacklisting.", &peer);
            tracing::debug!("Connection timeout for {}. Caused by: \n\t{:#?}", &peer, e);

            database.increment_attempts_since_last_seen(&peer).await?;
            // database.blacklist_peer(peer).await?;
        }
        Err(PeerCommunicationError::ContentDecodeError(e)) => {
            tracing::warn!(
                "Peer {} response could not be properly decoded. Blacklisting peer.",
                &peer,
            );
            tracing::debug!("Peer {} decoding error. Caused by:\n\t{:#?}", &peer, e);
            database.blacklist_peer(&peer).await?;
        }
        Err(PeerCommunicationError::UnexpectedError(e)) => {
            tracing::error!(
                "Problem getting peer info for {}. Caused by:\n\t{:#?}",
                &peer,
                e
            );

            database.increment_attempts_since_last_seen(&peer).await?;
        }
    }

    Ok(())
}

async fn get_peers(peer_address: &PeerAddress) -> Result<Vec<PeerAddress>, anyhow::Error> {
    let thebody = json!({
        "protocol": "B1",
        "requestType": "getPeers",
    });

    let response = post_peer_request(&thebody, None).await?;

    tracing::trace!("Parsing peers...");
    #[derive(Debug, serde::Deserialize)]
    struct PeerContainer {
        #[serde(rename = "peers")]
        peers: Vec<PeerAddress>,
    }
    let result = response.json::<PeerContainer>().await?;
    tracing::trace!("Peers successfully parsed: {:#?}", &result);
    Ok(result.peers)
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
    let _response = database.blacklist_peer(&peer).await?;
    Ok(())
}

/// De-blacklist a node. This should happen anytime this node queries it and receives
/// a correct response, or if it talks to this node with a correct introduction.
pub async fn deblacklist_peer(database: Datastore, peer: PeerAddress) -> Result<()> {
    let _response = database.deblacklist_peer(peer);
    Ok(())
}
