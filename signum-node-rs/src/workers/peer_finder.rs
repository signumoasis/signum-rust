use std::str::FromStr;

use anyhow::Result;

use crate::{configuration::Settings, get_db_pool, models::p2p::PeerAddress};

/// This worker finds new peers by querying the existing peers in the database.
/// If no peers exist in the database, it will read from the configuration bootstrap
/// peers list.
#[tracing::instrument(name = "Peer Finder", skip(settings))]
pub async fn peer_finder(settings: Settings) -> Result<()> {
    let db_pool = get_db_pool(&settings.database);
    let mut transaction = db_pool.begin().await?;
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

    if let Some(r) = row {
        let peer = PeerAddress::from_str(r.peer_address.as_str())?;
        tracing::debug!("Randomly chosen peer is {:#?}", peer);
    } else {
        tracing::debug!("No available peers in the database. Trying the bootstrap list.");
    }

    // If unable to get peer, try bootstrap peers
    Ok(())
}
