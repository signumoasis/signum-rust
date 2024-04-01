use std::time::Duration;

use anyhow::{Context, Result};
use surrealdb::{engine::any::Any, Surreal};
use tracing::Instrument;
use uuid::Uuid;

use crate::{
    configuration::Settings,
    models::p2p::PeerAddress,
    peers::{get_peers, update_db_peer_info},
};

pub async fn run_peer_finder_forever(database: Surreal<Any>, settings: Settings) -> Result<()> {
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
pub async fn peer_finder(database: Surreal<Any>, settings: Settings) -> Result<()> {
    tracing::info!("Seeking new peers");

    // Try to get random peer from database
    let mut response = database
        .query(
            r#"
                SELECT announced_address
                FROM ONLY peer
                WHERE blacklist.until IS none
                    OR blacklist.until < time::now()
                ORDER BY rand()
                LIMIT 1
            "#,
        )
        .await?;

    // Check if we were able to get a row
    let peer_address = response
        .take::<Option<PeerAddress>>("announced_address")
        .unwrap_or(None);

    // Extract the first element of the vec
    // let peer_address = if let Ok(the_vec) = result {
    //     the_vec.first().cloned()
    // } else {
    //     tracing::debug!("Couldn't get valid peer from database.");
    //     None
    // };

    // Check if we got a row AND were able to parse it
    let peer_address = if let Some(peer_address) = peer_address {
        // Use address from database
        peer_address
    } else {
        // Try address from bootstrap
        let peer = settings
            .p2p
            .bootstrap_peers
            .first()
            .ok_or_else(|| anyhow::anyhow!("Unable to get peer"))?;
        tracing::debug!("Trying the bootstrap list.");
        peer.to_owned()
    };

    tracing::debug!("Randomly chosen peer is {}", peer_address);
    // Next, send a request to that peer asking for its peers list.
    let peers = get_peers(peer_address)
        .await
        .context("unable to get peers from peer")?;

    let mut new_peers_count = 0;
    for peer in peers {
        tracing::trace!("Trying to save peer {}", peer);
        let response = database
            .query(
                r#"
                CREATE peer
                CONTENT {
                    announced_address: $announced_address
                }
            "#,
            )
            .bind(("announced_address", &peer))
            .await;

        match response {
            Ok(mut r) => {
                if let Ok(_) = r.take::<Vec<String>>("announced_address") {
                    tracing::debug!("Saved new peer {}", &peer);
                    tracing::debug!("Attempting to update peer info database for '{}'", &peer);
                    tokio::spawn(update_db_peer_info(database.clone(), peer).in_current_span());
                    new_peers_count += 1;
                } else {
                    tracing::debug!("Already have peer {}", peer)
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
