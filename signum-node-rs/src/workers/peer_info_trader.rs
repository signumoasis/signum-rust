use std::time::Duration;

use anyhow::{Context, Result};
use surrealdb::{engine::any::Any, Surreal};
use tracing::Instrument;
use uuid::Uuid;

use crate::{models::p2p::PeerAddress, peers::update_db_peer_info};

pub async fn run_peer_info_trader_forever(database: Surreal<Any>) -> Result<()> {
    loop {
        // Open the job-level span here so we also include the job_id in the error message if this result comes back Error.
        let span = tracing::span!(
            tracing::Level::INFO,
            "Peer Info Trade Task",
            job_id = Uuid::new_v4().to_string()
        );
        let result = peer_info_trader(database.clone()).instrument(span).await;
        if result.is_err() {
            tracing::error!("Error in peer info trader: {:?}", result);
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
/// Gets info from peer nodes and stores it.
/// Simultaneously supplies this node's info to the peers it contacts.
#[tracing::instrument(name = "Peer Info Trader", skip_all)]
pub async fn peer_info_trader(database: Surreal<Any>) -> Result<()> {
    // Get all peers from the database that haven't been seen in 1 minute
    let mut response = database
        .query(
            r#"
            SELECT announced_address
            FROM peer
            WHERE
                blacklist.until IS NULL OR blacklist.until < time::now()
                AND (last_seen is NONE OR last_seen is NULL OR last_seen < time::now() + 1m
        "#,
        )
        .await
        .context("unable to fetch peers from the database")?;

    let peers = response.take::<Vec<PeerAddress>>("announced_address")?;

    tracing::info!("Refreshing {} known peers", &peers.len());

    // Loop through the list to attempt to update the info for each one
    for peer in peers {
        tracing::debug!("Launching update task for {}", &peer);
        // Spawn update info task
        tokio::spawn(update_db_peer_info(database.clone(), peer).in_current_span());
    }

    Ok(())
}
