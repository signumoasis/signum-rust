use anyhow::Result;
use std::time::Duration;
use surrealdb::{engine::any::Any, Surreal};
use tracing::Instrument;
use uuid::Uuid;

use crate::configuration::Settings;

pub async fn run_block_downloader_forever(
    database: Surreal<Any>,
    settings: Settings,
) -> Result<()> {
    loop {
        // Open the job-level span here so we also include the job_id in the error message if this result comes back Error.
        let span = tracing::span!(
            tracing::Level::INFO,
            "Block Downloader",
            job_id = Uuid::new_v4().to_string()
        );
        let result = block_downloader(database.clone(), settings.clone())
            .instrument(span)
            .await;
        if result.is_err() {
            tracing::error!("Error in block downloader: {:?}", result);
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

/// This worker finds new peers by querying the existing peers in the database.
/// If no peers exist in the database, it will read from the configuration bootstrap
/// peers list.
#[tracing::instrument(name = "Block Downloader", skip_all)]
pub async fn block_downloader(database: Surreal<Any>, settings: Settings) -> Result<()> {
    tracing::info!("Block Downloader Tick");
    Ok(())
}
