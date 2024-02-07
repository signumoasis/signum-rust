use std::{str::FromStr, time::Duration};

use anyhow::{Context, Result};
use sqlx::SqlitePool;

use crate::{configuration::Settings, models::p2p::PeerAddress, peers::get_peers};

pub async fn run_peer_finder_forever(pool: SqlitePool, settings: Settings) -> Result<()> {
    loop {
        let result = peer_finder(pool.clone(), settings.clone()).await;
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
pub async fn peer_finder(pool: SqlitePool, settings: Settings) -> Result<()> {
    tracing::info!("Seeking new peers");
    let mut transaction = pool
        .begin()
        .await
        .context("unable to get transaction from pool")?;
    // Try to get random peer from database
    let row = sqlx::query!(
        r#"
            SELECT peer_announced_address
            FROM peers
            WHERE blacklist_until IS NULL or blacklist_until < DATETIME('now')
            ORDER BY RANDOM()
            LIMIT 1;
        "#
    )
    .fetch_optional(&mut *transaction)
    .await?;

    // Check if we were able to get a row
    let x = if let Some(r) = row {
        PeerAddress::from_str(r.peer_announced_address.as_str())
    } else {
        let err = anyhow::anyhow!("No valid peers available in the database.");
        tracing::debug!("Couldn't get peer from database: {}", err);
        Err(err)
    };

    // Check if we got a row AND were able to parse it
    let peer = if let Ok(peer_address) = x {
        // Use address from database
        peer_address
    } else {
        // Try address from bootstrap
        let peer = settings
            .p2p
            .bootstrap_peers
            // TODO: Make this selection random
            .get(0)
            .ok_or_else(|| anyhow::anyhow!("Unable to get peer"))?;
        tracing::debug!("Trying the bootstrap list.");
        peer.to_owned()
    };

    tracing::debug!("Randomly chosen peer is {:#?}", peer);
    // Next, send a request to that peer asking for its peers list.
    let peers = get_peers(peer)
        .await
        .context("unable to get peers from database")?;

    // Insert the peers into the database, silently ignoring if they fail
    // due to the unique requirement for primary key
    let mut new_peers_count = 0;
    for peer in peers {
        tracing::trace!("Trying to save peer {}", peer);
        let result = sqlx::query!(
            r#" INSERT OR IGNORE
            INTO peers (peer_announced_address)
            VALUES ($1)
        "#,
            peer
        )
        .execute(&mut *transaction)
        .await;

        match result {
            Ok(r) => {
                let number = r.rows_affected();
                new_peers_count += number;
                if number >= 1 {
                    tracing::debug!("Saving new peer {}", peer);
                } else {
                    tracing::debug!("Already have peer {}", peer)
                }
            }
            Err(e) => {
                tracing::error!("Unable to save peer: {:?}", e);
                continue;
            }
        }
    }
    transaction
        .commit()
        .await
        .context("unable to commit transaction -- peers not saved")?;
    tracing::info!("Added {} new peers.", new_peers_count);
    Ok(())
}
