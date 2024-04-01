use std::collections::HashMap;

use actix_web::ResponseError;
use anyhow::{Context, Result};
use surrealdb::{
    engine::any::Any,
    sql::statements::{BeginStatement, CommitStatement},
    Surreal,
};

use crate::models::p2p::{PeerAddress, PeerInfo};

pub async fn get_peers(peer: PeerAddress) -> Result<Vec<PeerAddress>, anyhow::Error> {
    let mut thebody = HashMap::new();
    thebody.insert("protocol", "B1");
    thebody.insert("requestType", "getPeers");

    let peer_request = reqwest::Client::new()
        .post(peer.to_url())
        .header("User-Agent", "BRS/3.8.0")
        .json(&thebody)
        .send()
        .await?;

    tracing::trace!("Parsing peers...");
    #[derive(Debug, serde::Deserialize)]
    struct PeerContainer {
        #[serde(rename = "peers")]
        peers: Vec<PeerAddress>,
    }
    let response = peer_request.json::<PeerContainer>().await?;
    tracing::trace!("Peers successfully parsed: {:#?}", &response);
    Ok(response.peers)
}

/// Requests peer information from the the supplied PeerAddress. Updates the database
/// with the acquired information. Returns a [`anyhow::Result<()>`].
#[tracing::instrument(name = "Update Info Task", skip_all)]
pub async fn update_db_peer_info(database: Surreal<Any>, peer: PeerAddress) -> Result<()> {
    let peer_info = get_peer_info(peer.clone()).await;
    match peer_info {
        Ok(info) => {
            tracing::trace!("PeerInfo: {:?}", &info);

            let ip = info.1;
            let info = info.0;

            let response = database
                .query(BeginStatement)
                .query(
                    r#"
                        UPDATE peer
                        MERGE {
                            announced_address: $new_announced_address,
                            ip_address: $ip_address,
                            application: $application,
                            version: $version,
                            platform: $platform,
                            share_address: $share_address,
                            network: $network,
                            last_seen: time::now(),
                            attempts_since_last_seen: 0
                        }
                        WHERE announced_address = $announced_address
                    "#,
                )
                .bind(("announced_address", &peer))
                .bind(("new_announced_address", info.announced_address))
                .bind(("ip_address", ip))
                .bind(("application", info.application))
                .bind(("version", info.version))
                .bind(("platform", info.platform))
                .bind(("share_address", info.share_address))
                .bind(("network", info.network_name));

            let _response = response
                .query(CommitStatement)
                .await
                .context(format!("unable to update peer info for {}", &peer))?;

            // TODO: Consider removing this to avoid deblacklisting a peer providing bad blocks
            deblacklist_peer(database, peer).await?;
        }
        Err(GetPeerInfoError::MissingAnnouncedAddress(_)) => {
            tracing::warn!(
                "Peer {} has no 'announcedAddress' configured. Blacklisting.",
                &peer
            );
            blacklist_peer(database, peer).await?;
        }
        Err(GetPeerInfoError::UnexpectedError(e)) => {
            tracing::error!("Problem getting peer info for {}: {:?}", &peer, e);
            increment_attempts_since_last_seen(database, peer).await?;
        }
        Err(GetPeerInfoError::ConnectionError(e)) => {
            tracing::warn!(
                "Connection error to peer {}. Blacklisting. Caused by:\n\t{}",
                &peer,
                e
            );
            tracing::trace!("Connection error for {}: Caused by:\n\t{:#?}", &peer, e);
            increment_attempts_since_last_seen(database.clone(), peer.clone()).await?;
            // TODO: Consider only blacklisting after several consecutive bad attempts. 120 minutes?
            blacklist_peer(database, peer).await?;
        }
        Err(GetPeerInfoError::ConnectionTimeout(e)) => {
            tracing::warn!("Connection to peer {} has timed out. Blacklisting.", &peer);
            tracing::trace!("Timeout caused by: {:#?}", e);

            increment_attempts_since_last_seen(database.clone(), peer.clone()).await?;
            // TODO: Consider only blacklisting after several consecutive bad attempts. 120 minutes?
            blacklist_peer(database, peer).await?;
        }
    }

    Ok(())
}

pub async fn increment_attempts_since_last_seen(
    database: Surreal<Any>,
    peer: PeerAddress,
) -> Result<()> {
    let _response = database
        .query(BeginStatement)
        .query(
            r#"
                UPDATE peer
                SET attempts_since_last_seen += 1
                WHERE announced_address = $peer
            "#,
        )
        .bind(("peer", &peer))
        .query(CommitStatement)
        .await
        .context(format!(
            "could not increment attempts_since_last_seen for {}",
            &peer
        ))?;
    Ok(())
}

/// Makes an http request to the supplied peer address and parses the returned information
/// into a [`PeerInfo`].
///
/// Returns a tuple of ([`PeerInfo`], [`String`]) where the string is the resolved IP
/// address of the peer.
#[tracing::instrument]
pub async fn get_peer_info(peer: PeerAddress) -> Result<(PeerInfo, String), GetPeerInfoError> {
    let mut thebody = HashMap::new();
    thebody.insert("protocol", "B1");
    thebody.insert("requestType", "getInfo");
    thebody.insert("announcedAddress", "nodomain.com");
    thebody.insert("application", "BRS");
    thebody.insert("version", "3.8.0");
    thebody.insert("platform", "signum-rs");
    thebody.insert("shareAddress", "false");

    let response = reqwest::Client::new()
        .post(peer.to_url())
        .header("User-Agent", "BRS/3.8.0")
        .json(&thebody)
        .send()
        .await;

    let response = match response {
        Ok(r) => Ok(r),
        Err(e) if e.is_connect() => Err(GetPeerInfoError::ConnectionError(e)),
        Err(e) if e.is_timeout() => Err(GetPeerInfoError::ConnectionTimeout(e)),
        Err(e) => Err(GetPeerInfoError::UnexpectedError(
            Err(e).context("could not get a response")?,
        )),
    }?;

    //TODO: Think about letting this be optional here and fix it in the future requests
    let peer_ip = response
        .remote_addr()
        .ok_or_else(|| anyhow::anyhow!("peer response did not have an IP address"))?
        .ip()
        .to_string();

    tracing::trace!("Found IP address {} for PeerAddress {}", &peer_ip, &peer);

    let peer_info = match response.json::<PeerInfo>().await {
        Ok(i) => Ok(i),
        Err(e) => Err(GetPeerInfoError::MissingAnnouncedAddress(e)),
    }?;

    Ok((peer_info, peer_ip))
}

#[derive(thiserror::Error)]
pub enum GetPeerInfoError {
    #[error("Missing announced address: {0}")]
    MissingAnnouncedAddress(#[source] reqwest::Error),
    #[error("Connection error {0}")]
    ConnectionError(#[source] reqwest::Error),
    #[error("Connection timeout {0}")]
    ConnectionTimeout(#[source] reqwest::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for GetPeerInfoError {}

impl std::fmt::Debug for GetPeerInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::error_chain_fmt(self, f)
    }
}

/// Blacklist a client for minutes * blacklist_count, for a maximum of 24 hours.
/// blacklist_count increments by 1 each time a node is blacklisted, so it will
/// be ignored for longer and longer, up to 24 hours before retry.
pub async fn blacklist_peer(database: Surreal<Any>, peer: PeerAddress) -> Result<()> {
    let blacklist_base_minutes = 10;
    let _response = database
        .query(BeginStatement)
        .query(
            r#"
                UPDATE peer
                SET
                    blacklist.count += 1,
                    blacklist.until = time::now() + type::duration(string::concat(math::min([$blacklist_base_minutes * (blacklist.count + 1),1440]),"m"))
                    WHERE announced_address = $peer
            "#,
        )
        .bind(("blacklist_base_minutes", blacklist_base_minutes))
        .bind(("peer", &peer))
        .query(CommitStatement)
        .await
        .context(format!(
            "could not blacklist {}",
            &peer
        ))?;
    Ok(())
}

/// De-blacklist a node. This should happen anytime this node queries it and receives
/// a correct response, or if it talks to this node with a correct introduction.
pub async fn deblacklist_peer(database: Surreal<Any>, peer: PeerAddress) -> Result<()> {
    let _response = database
        .query(BeginStatement)
        .query(
            r#"
                UPDATE peer
                SET
                    blacklist.count = 0,
                    blacklist.until = null,
                    WHERE announced_address = $peer
            "#,
        )
        .bind(("peer", &peer))
        .query(CommitStatement)
        .await
        .context(format!("could not deblacklist {}", &peer))?;
    Ok(())
}
