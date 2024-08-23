use std::time::Duration;

use actix_web::ResponseError;
use anyhow::{Context, Result};
use surrealdb::{
    engine::any::Any,
    sql::statements::{BeginStatement, CommitStatement},
    Response, Surreal,
};

use super::p2p::{PeerAddress, PeerInfo};

#[derive(Clone, Debug)]
pub struct Datastore {
    db: Surreal<Any>,
}

impl Datastore {
    /// Instantiates a new Datastore containing a [`Surreal`]<[`Any`]>.
    pub fn new(db: Surreal<Any>) -> Self {
        Self { db }
    }

    /// Returns a clone of the raw Surrealdb handle for use with custom queries.
    ///
    /// For when the Datastore class just won't do.
    pub fn get_surreal_db(&self) -> Surreal<Any> {
        self.db.clone()
    }

    /// Blacklists the provided peer. It will begin with 10 minutes and keeps track of the number
    /// of times the peer was blacklisted. For each instance, the timer grows by 10 minutes until a
    /// maximum of 24 hours.
    ///
    /// 1st blacklist: 10 minutes,
    /// 2nd blacklist: 20 minutes,
    /// 3rd blacklist: 30 minutes,
    /// etc. until a maximum of 24 hours at a time.
    pub async fn blacklist_peer(&self, peer: PeerAddress) -> Result<Response, DatastoreError> {
        let blacklist_base_minutes = 10;

        let response = self.db
        .query(BeginStatement::default())
        .query(
            r#"
                UPDATE peer
                SET
                    blacklist.count += 1,
                    blacklist.until = time::now() + type::duration(string::concat(math::min([$blacklist_base_minutes * (blacklist.count + 1),1440]),"m"))
                    WHERE announced_address = $peer
            "#,
        )
        .bind(("blacklist_base_minutes", blacklist_base_minutes))
        .bind(("peer", peer.clone()))
        .query(CommitStatement::default())
        .await
        .context(format!(
            "could not blacklist {}",
            &peer
        ))?;

        Ok(response)
    }

    /// Adds a new peer to the database.
    pub async fn create_new_peer(
        &mut self,
        peer: &PeerAddress,
    ) -> Result<Response, DatastoreError> {
        let response = self
            .db
            .query(
                r#"
                CREATE peer
                CONTENT {
                    announced_address: $announced_address
                }
            "#,
            )
            .bind(("announced_address", peer.clone()))
            .await
            .context("could not create a new peer in the database")?;
        Ok(response)
    }

    /// Manually deblacklists a peer.
    pub async fn deblacklist_peer(&self, peer: PeerAddress) -> Result<Response, DatastoreError> {
        let response = self
            .db
            .query(BeginStatement::default())
            .query(
                r#"
                UPDATE peer
                SET
                    blacklist.count = 0,
                    blacklist.until = null,
                    WHERE announced_address = $peer
            "#,
            )
            .bind(("peer", peer.clone()))
            .query(CommitStatement::default())
            .await
            .context(format!("could not deblacklist {}", &peer))?;
        Ok(response)
    }

    /// Returns a list of peers whose last seen time is older than the [`Duration`].
    pub async fn get_peers_last_seen_before(&self, duration: Duration) -> Result<Vec<PeerAddress>> {
        let mut response = self
            .db
            .query(
                r#"
            SELECT announced_address
            FROM peer
            WHERE
                blacklist.until IS NULL OR blacklist.until < time::now()
                AND (last_seen is NONE OR last_seen is NULL OR last_seen < $duration)
        "#,
            )
            .bind(("duration", duration))
            .await
            .context("unable to fetch peers from the database")?;

        let peers = response.take::<Vec<PeerAddress>>("announced_address")?;

        Ok(peers)
    }

    /// Returns a randomized peer from the database.
    ///
    /// Returns an error if there was a problem or if there are no peers in the database.
    pub async fn get_random_peer(&mut self) -> Result<PeerAddress, DatastoreError> {
        let mut response = self
            .db
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
            .await
            .context("unable to get a random peer from the database")?;

        // Check if we were able to get a row
        let peer_address = response
            .take::<Option<PeerAddress>>("announced_address")
            .context("unable to deserialize the peer from the response")?;

        Ok(peer_address.ok_or_else(|| anyhow::anyhow!("no random address could be found"))?)
    }

    /// Returns up to the requested number of random [`PeerAddress`]es from the database.
    ///
    /// Returns an error if there was a problem or if there are no peers in the database.
    pub async fn get_n_random_peers(
        &mut self,
        number: u32,
    ) -> Result<Vec<PeerAddress>, DatastoreError> {
        let mut response = self
            .db
            .query(
                r#"
                SELECT announced_address
                FROM peer
                WHERE blacklist.until IS none
                    OR blacklist.until < time::now()
                ORDER BY rand()
                LIMIT $number
            "#,
            )
            .bind(("number", number))
            .await
            .context("unable to get random peers from the database")?;

        // Check if we were able to get a row
        let peer_address = response
            .take::<Vec<PeerAddress>>("announced_address")
            .context("unable to deserialize the peer from the response")?;
        if peer_address.is_empty() {
            return Err(anyhow::anyhow!("no random addresses could be found"))?;
        }
        Ok(peer_address)
    }

    /// Increments the number of attempts to contact a peer since a peer was last seen.
    pub async fn increment_attempts_since_last_seen(
        &self,
        peer: PeerAddress,
    ) -> Result<Response, DatastoreError> {
        let response = self
            .db
            .query(BeginStatement::default())
            .query(
                r#"
                UPDATE peer
                SET attempts_since_last_seen += 1
                WHERE announced_address = $peer
            "#,
            )
            .bind(("peer", peer.clone()))
            .query(CommitStatement::default())
            .await
            .context(format!(
                "could not increment attempts_since_last_seen for {}",
                &peer
            ))?;
        Ok(response)
    }

    /// Provide a [`PeerInfo`] to update a peer's information.
    pub async fn update_peer_info(
        &self,
        peer_address: PeerAddress,
        new_ip_address: String,
        peer_info: PeerInfo,
    ) -> Result<Response, DatastoreError> {
        let response = self
            .db
            .query(BeginStatement::default())
            .query(
                r#"
                        UPDATE peer
                        MERGE {
                            announced_address: $new_announced_address,
                            ip_address: $ip_address,
                            application: $application,
                            version: $version,
                            platform: $platform,
                            share_address: $share_address,
                            network: $network,
                            last_seen: time::now(),
                            attempts_since_last_seen: 0
                        }
                        WHERE announced_address = $announced_address
                    "#,
            )
            .bind(("announced_address", peer_address.clone()))
            .bind(("new_announced_address", peer_info.announced_address))
            .bind(("ip_address", new_ip_address))
            .bind(("application", peer_info.application))
            .bind(("version", peer_info.version))
            .bind(("platform", peer_info.platform))
            .bind(("share_address", peer_info.share_address))
            .bind(("network", peer_info.network_name));

        let response = response
            .query(CommitStatement::default())
            .await
            .context(format!("unable to update peer info for {}", peer_address))?;

        Ok(response)
    }
}

/// Represents a Datastore error.
#[derive(thiserror::Error)]
pub enum DatastoreError {
    /// An unexpected error. Will contain an [`anyhow::Error`] with additional details.
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for DatastoreError {}

impl std::fmt::Debug for DatastoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::error_chain_fmt(self, f)
    }
}
