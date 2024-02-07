use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::SqlitePool;

use crate::{
    models::p2p::PeerAddress,
    peers::{blacklist_peer, deblacklist_peer, get_peer_info, GetPeerInfoError},
};

pub async fn run_peer_info_trader_forever(
    read_pool: SqlitePool,
    write_pool: SqlitePool,
) -> Result<()> {
    loop {
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

    // Loop through the list to attempt to get the info for each one
    for peer in peers {
        tracing::trace!("Launching update task for {}", &peer);
        // Spawn update info task
        tokio::spawn(update_info_task(write_pool.clone(), peer));
    }

    Ok(())
}

#[tracing::instrument(name = "Update Info Task", skip_all)]
pub async fn update_info_task(write_pool: SqlitePool, peer: PeerAddress) -> Result<()> {
    let peer_info = get_peer_info(peer.clone()).await;
    match peer_info {
        Ok(info) => {
            tracing::trace!("PeerInfo: {:?}", &info);

            let ip = info.1;
            let info = info.0;

            let mut transaction = write_pool.begin().await?;

            let r = sqlx::query!(
                r#"
                    UPDATE peers
                    SET
                        peer_announced_address = $1,
                        peer_ip_address = $2,
                        application = $3,
                        version = $4,
                        platform = $5,
                        share_address = $6,
                        network = $7,
                        last_seen = DATETIME('now')
                    WHERE
                        peer_announced_address = $8
                "#,
                info.announced_address,
                ip,
                info.application,
                info.version,
                info.platform,
                info.share_address,
                info.network_name,
                peer
            )
            .execute(&mut *transaction)
            .await
            .context(format!("unable to update peer info for {}", &peer))?;

            if r.rows_affected() == 0 {
                anyhow::bail!(
                    "no error occurred but peer {} was not updated for some reason",
                    &peer
                );
            }

            transaction.commit().await?;

            // TODO: Consider removing this to avoid deblacklisting a peer providing bad blocks
            deblacklist_peer(write_pool, peer).await?;
        }
        Err(GetPeerInfoError::MissingAnnouncedAddress(_)) => {
            tracing::warn!(
                "Peer {} has no 'announcedAddress' configured. Blacklisting.",
                &peer
            );
            blacklist_peer(write_pool, peer).await?;
        }
        Err(GetPeerInfoError::UnexpectedError(e)) => {
            tracing::error!("Problem getting per info for {}: {:?}", &peer, e);
        }
        Err(GetPeerInfoError::ConnectionTimeout(e)) => {
            tracing::warn!("Connection to peer {} has timed out. Blacklisting.", &peer);
            tracing::trace!("Timeout caused by: {:#?}", e);
            blacklist_peer(write_pool, peer).await?;
        }
    }

    Ok(())
}
