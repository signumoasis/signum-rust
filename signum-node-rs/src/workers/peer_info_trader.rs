use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::SqlitePool;

use crate::PeerAddress;

use crate::get_peer_info;

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
#[tracing::instrument(name = "Peer Info Trader", skip_all)]
pub async fn peer_info_trader(pool: SqlitePool) -> Result<()> {
    // Get all peers from the database that haven't been seen in 1 minute
    let peers = sqlx::query!(
        r#"
        SELECT peer_announced_address
        FROM peers
        WHERE last_seen is NULL
            OR last_seen < DateTime('now', '+1 minute')
        "#
    )
    .fetch_all(&pool.clone())
    .await
    .context("unable to fetch peers from databse")?
    .iter()
    .map(|row| PeerAddress::from_str(&row.peer_announced_address))
    .collect::<Result<Vec<PeerAddress>, _>>()?;

    tracing::info!("Refreshing {} known peers", &peers.len());

    // Loop through the list to attempt to get the info for each one
    for peer in peers {
        tracing::trace!("Launching update task for {}", &peer);
        // Spawn update info task
        tokio::spawn(update_info_task(pool.clone(), peer));
    }

    Ok(())
}

#[tracing::instrument(name = "Update Info Task", skip_all)]
pub async fn update_info_task(_pool: SqlitePool, peer: PeerAddress) -> Result<()> {
    let peer_info = get_peer_info(peer.clone()).await.context("Unable to get peer info");
    match peer_info {
        Ok(info) => {
            tracing::debug!("PeerInfo: {:?}", info);
        }
        Err(e) => {
            tracing::error!("Problem getting peer info for {}: {:?}", &peer, e)
        }
    }

    Ok(())
}
