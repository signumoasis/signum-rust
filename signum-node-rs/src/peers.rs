use std::collections::HashMap;

use anyhow::{Context, Result};
use sqlx::SqlitePool;

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

    tracing::debug!("Parsing peers");
    #[derive(Debug, serde::Deserialize)]
    struct PeerContainer {
        #[serde(rename = "peers")]
        peers: Vec<PeerAddress>,
    }
    let response = peer_request.json::<PeerContainer>().await?;
    Ok(response.peers)
}

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
        .await
        .context("unable to connect to peer")?;

    //TODO: Check for timeouts and forward that error type

    //TODO: Think about letting this be optional here and fix it in the future requests
    let peer_ip = response
        .remote_addr()
        .ok_or_else(|| anyhow::anyhow!("peer response did not have an IP address"))?
        .ip()
        .to_string();

    tracing::trace!("Found IP address {} for PeerAddress {}", &peer_ip, &peer);

    let peer_info = response.json::<PeerInfo>().await?;

    Ok((peer_info, peer_ip))
}

#[derive(thiserror::Error)]
pub enum GetPeerInfoError {
    #[error(transparent)]
    MissingAnnouncedAddress(#[from] reqwest::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for GetPeerInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::error_chain_fmt(self, f)
    }
}

pub async fn blacklist_peer(pool: SqlitePool, peer: PeerAddress) -> Result<()> {
    let mut transaction = pool.begin().await?;

    let r = sqlx::query!(
        r#"
            UPDATE peers
            SET
                blacklist_until = DATETIME('now','+' || $1 || ' minutes'),
                blacklist_count = blacklist_count + 1,
                last_seen = DATETIME('now')
            WHERE peer_announced_address = $2
        "#,
        10,
        peer
    )
    .execute(&mut *transaction)
    .await
    .context(format!("could not blacklist {}", &peer))?;
    if r.rows_affected() == 0 {
        anyhow::bail!(
            "no error occurred but {} was not blacklisted for some reason",
            &peer
        );
    }
    transaction.commit().await?;
    Ok(())
}

pub async fn deblacklist_peer(pool: SqlitePool, peer: PeerAddress) -> Result<()> {
    let mut transaction = pool.begin().await?;

    let r = sqlx::query!(
        r#"
            UPDATE peers
            SET
                blacklist_until = NULL,
                blacklist_count = 0
            WHERE peer_announced_address = $1
        "#,
        peer
    )
    .execute(&mut *transaction)
    .await
    .context(format!("could not deblacklist {}", &peer))?;
    if r.rows_affected() == 0 {
        anyhow::bail!(
            "no error occurred but {} was not deblacklisted for some reason",
            &peer
        );
    }
    transaction.commit().await?;
    Ok(())
}
