use std::time::Duration;

use anyhow::{Context, Result};
use tracing::Instrument;
use uuid::Uuid;

use crate::{
    configuration::Settings,
    models::datastore::Datastore,
    peers::{update_db_peer_info, B1Peer, BasicPeerClient},
};

pub async fn run_peer_finder_forever(database: Datastore, settings: Settings) -> Result<()> {
    loop {
        // Open the job-level span here so we also include the job_id in the error message if this result comes back Error.
        let span = tracing::span!(
            tracing::Level::INFO,
            "Peer Finder Task",
            job_id = Uuid::new_v4().to_string()
        );
        let result = peer_finder(database.clone(), settings.clone())
            .instrument(span)
            .await;
        if result.is_err() {
            tracing::error!("Error in peer finder: {:?}", result);
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

/// This worker finds new peers by querying the existing peers in the database.
/// If no peers exist in the database, it will read from the configuration bootstrap
/// peers list.
#[tracing::instrument(name = "Peer Finder", skip_all)]
pub async fn peer_finder(mut database: Datastore, settings: Settings) -> Result<()> {
    // Try to get random peer from database
    let peer_address = database.get_random_peer().await;

    // Check if we got a row AND were able to parse it
    let peer_address = if let Ok(peer_address) = peer_address {
        // Use address from database
        peer_address
    } else {
        // Try address from bootstrap
        //TODO: Make this a random selection instead of just taking the first one.
        let peer = settings
            .p2p
            .bootstrap_peers
            .first()
            .ok_or_else(|| anyhow::anyhow!("Unable to get peer"))?;
        tracing::debug!("Trying the bootstrap list.");
        peer.to_owned()
    };

    tracing::info!("Seeking new peers from {}", &peer_address);

    let peer = B1Peer::new(peer_address);

    // Next, send a request to that peer asking for its peers list.
    let peers = peer
        .get_peers()
        .await
        .context("unable to get peers from peer")?;

    let mut new_peers_count = 0;
    for peer_address in peers {
        tracing::trace!("Trying to save peer {}", peer_address);
        let response = database.create_new_peer(&peer_address).await;

        let peer = B1Peer::new(peer_address.clone());

        match response {
            Ok(mut r) => {
                if r.take::<Vec<String>>("announced_address").is_ok() {
                    tracing::debug!("Saved new peer {}", &peer_address);
                    tracing::debug!(
                        "Attempting to update peer info database for '{}'",
                        &peer_address
                    );
                    tokio::spawn(update_db_peer_info(database.clone(), peer).in_current_span());
                    new_peers_count += 1;
                } else {
                    tracing::debug!("Already have peer {}", peer_address)
                };
            }
            Err(e) => {
                tracing::error!("Unable to save peer: {:?}", e);
                continue;
            }
        }
    }

    tracing::info!("Added {} new peers.", new_peers_count);
    Ok(())
}
