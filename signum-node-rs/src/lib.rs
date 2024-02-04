use std::collections::HashMap;

use configuration::DatabaseSettings;
use models::p2p::{PeerAddress, PeerInfo};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub mod configuration;
pub mod models;
pub mod telemetry;
pub mod workers;

/// Get a database connection pool.
pub fn get_db_pool(configuration: &DatabaseSettings) -> SqlitePool {
    SqlitePoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.get_db())
}

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

pub async fn get_peer_info(peer: PeerAddress) -> Result<PeerInfo, anyhow::Error> {
    let mut thebody = HashMap::new();
    thebody.insert("protocol", "B1");
    thebody.insert("requestType", "getInfo");
    thebody.insert("announcedAddress", "nodomain.com");
    thebody.insert("application", "BRS");
    thebody.insert("version", "3.8.0");
    thebody.insert("platform", "signum-rs");
    thebody.insert("shareAddress", "false");

    let peer_request = reqwest::Client::new()
        .post(peer.to_url())
        .header("User-Agent", "BRS/3.8.0")
        .json(&thebody)
        .send()
        .await?;
    tracing::debug!("Parsing peer info");
    let response = peer_request.json::<PeerInfo>().await?;

    Ok(response)
}
