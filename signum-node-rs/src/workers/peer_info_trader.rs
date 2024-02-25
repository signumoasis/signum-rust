use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::SqlitePool;
use tracing::Instrument;
use uuid::Uuid;

use crate::{models::p2p::PeerAddress, peers::update_db_peer_info};

pub async fn run_peer_info_trader_forever(
    read_pool: SqlitePool,
    write_pool: SqlitePool,
) -> Result<()> {
    loop {
        // Open the job-level span here so we also include the job_id in the error message if this result comes back Error.
        let span = tracing::span!(
            tracing::Level::INFO,
            "Peer Info Trade Task",
            job_id = Uuid::new_v4().to_string()
        );
        let _guard = span.enter();
        let result = peer_info_trader(read_pool.clone(), write_pool.clone()).await;
        if result.is_err() {
            tracing::error!("Error in peer info trader: {:?}", result);
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
/// Gets info from peer nodes and stores it.
/// Simultaneously supplies this node's info to the peers it contacts.
#[tracing::instrument(name = "Peer Info Trader", skip_all)]
pub async fn peer_info_trader(read_pool: SqlitePool, write_pool: SqlitePool) -> Result<()> {
    // Get all peers from the database that haven't been seen in 1 minute
    let peers = sqlx::query!(
        r#"
        SELECT peer_announced_address
        FROM peers
        WHERE
            (blacklist_until IS NULL OR blacklist_until < DATETIME('now'))
            AND (last_seen is NULL OR last_seen < DateTime('now', '+1 minute'))
        "#
    )
    .fetch_all(&read_pool)
    .await
    .context("unable to fetch peers from databse")?
    .iter()
    .map(|row| PeerAddress::from_str(&row.peer_announced_address))
    .collect::<Result<Vec<PeerAddress>, _>>()?;

    tracing::info!("Refreshing {} known peers", &peers.len());

    // Loop through the list to attempt to update the info for each one
    for peer in peers {
        tracing::trace!("Launching update task for {}", &peer);
        // Spawn update info task
        tokio::spawn(update_db_peer_info(write_pool.clone(), peer).in_current_span());
    }

    Ok(())
}
