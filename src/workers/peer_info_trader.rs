use std::time::Duration;

use anyhow::Result;
use tracing::Instrument;
use uuid::Uuid;

use crate::{models::datastore::Datastore, peers::update_db_peer_info};

pub async fn run_peer_info_trader_forever(database: Datastore) -> Result<()> {
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
pub async fn peer_info_trader(database: Datastore) -> Result<()> {
    // Get all peers from the database that haven't been seen in 1 minute
    let peers = database
        .get_peers_last_seen_before(Duration::from_secs(60))
        .await?;

    tracing::info!("Refreshing {} known peers", &peers.len());

    // Loop through the list to attempt to update the info for each one
    for peer in peers {
        tracing::debug!("Launching update task for {}", &peer);
        // Spawn update info task
        tokio::spawn(update_db_peer_info(database.clone(), peer).in_current_span());
    }

    Ok(())
}
