use std::{str::FromStr, time::Duration};

use anyhow::{Context, Result};

use crate::{configuration::Settings, get_db_pool, get_peers, models::p2p::PeerAddress};

pub async fn run_peer_finder_forever(settings: Settings) {
    loop {
        let result = peer_finder(settings.clone()).await;
        if result.is_err() {
            tracing::error!("Error in peer finder: {:?}", result);
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

/// This worker finds new peers by querying the existing peers in the database.
/// If no peers exist in the database, it will read from the configuration bootstrap
/// peers list.
#[tracing::instrument(name = "Peer Finder", skip(settings))]
pub async fn peer_finder(settings: Settings) -> Result<()> {
    let db_pool = get_db_pool(&settings.database);
    let mut transaction = db_pool
        .begin()
        .await
        .context("unable to get transaction from pool")?;
    // Try to get random peer from database
    let row = sqlx::query!(
        r#"
            SELECT peer_address
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
        PeerAddress::from_str(r.peer_address.as_str())
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
    tracing::info!("Saving new peers to the database.");
    for peer in peers {
        tracing::trace!("Saving peer {}", peer);
        let result = sqlx::query!(
            r#" INSERT OR IGNORE
            INTO peers (peer_address)
            VALUES ($1)
        "#,
            peer
        )
        .execute(&mut *transaction)
        .await;
        tracing::trace!("RESULT: {:?}", result);
        if result.is_err() {
            tracing::error!("Unable to save peer: {:?}", result);
            continue;
        }
    }
    let _ = transaction.commit().await;
    Ok(())
}
