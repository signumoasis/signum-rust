use std::time::Duration;

use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::p2p::PeerInfo;

pub async fn run_peer_info_trader_forever(pool: SqlitePool) -> Result<()> {
    loop {
        let result = peer_info_trader(pool.clone()).await;
        if result.is_err() {
            tracing::error!("Error in peer info trader: {:?}", result);
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
/// Gets info from peer nodes and stores it.
/// Simultaneously supplies this node's info to the peers it contacts.
pub async fn peer_info_trader(pool: SqlitePool) -> Result<()> {
    // Get all peers from the database that haven't been seen in 1 minute
    let peers = Vec::<PeerInfo>::new();
    // Loop through the list to attempt to get the info for each one
    for peer in peers {
        // Spawn update info task
        tokio::spawn(update_info_task(peer));
    }

    Ok(())
}

pub async fn update_info_task(peer: PeerInfo) -> Result<PeerInfo> {
    todo!()
}
