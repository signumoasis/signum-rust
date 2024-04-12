use anyhow::Result;
use serde_json::json;
use std::{collections::VecDeque, time::Duration};
use tracing::Instrument;
use uuid::Uuid;

use crate::{
    configuration::Settings,
    models::{
        datastore::Datastore,
        p2p::{B1Block, PeerAddress},
    },
};

pub async fn run_block_downloader_forever(database: Datastore, settings: Settings) -> Result<()> {
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
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

/// This worker queries random peers for new blocks.
#[tracing::instrument(name = "Block Downloader", skip_all)]
pub async fn block_downloader(mut database: Datastore, _settings: Settings) -> Result<()> {
    tracing::info!("Would download blocks");
    // Steps:
    // * Initiate FIFO queue
    // * Determine highest block stored in DB
    // * Determine the mode of the highest cumulative difficulty
    //   of several or maybe all known peers
    // * Build a list of peers with the discovered cumulative difficulty
    //
    // NOTES:
    // Things to keep track of:
    // * Which block sets have been queried - handled by FIFO task queue
    // * Which ones have errored - handled by FIFO task queue, but must be re-requested
    //
    let _downloads = VecDeque::<B1Block>::new();

    let _highest_cumulative_difficulty = 0u128;
    let peers = database.get_n_random_peers(5).await?;

    tracing::debug!("Random peers from db: {:#?}", &peers);

    let _cumulative_difficulties: Vec<u128> = Vec::new();

    for _peer in peers {}

    Ok(())
}

async fn download_blocks_task(peer: PeerAddress, height: u64, number_of_blocks: u64) -> Result<()> {
    let thebody = json!({
        "protocol": "B1",
        "requestType": "getBlocksFromHeight",
        "height": height,
        "numBlocks": number_of_blocks,
    });
    // let mut thebody = HashMap::new();
    // thebody.insert("protocol", "B1");
    // thebody.insert("requestType", "getBlocksFromHeight");
    // thebody.insert("height", height);

    let peer_request = reqwest::Client::new()
        .post(peer.to_url())
        .header("User-Agent", "BRS/3.8.0")
        .json(&thebody)
        .send()
        .await?;

    tracing::trace!("Parsing peers...");
    #[derive(Debug, serde::Deserialize)]
    struct PeerContainer {
        #[serde(rename = "peers")]
        peers: Vec<PeerAddress>,
    }
    let response = peer_request.json::<PeerContainer>().await?;
    tracing::trace!("Peers successfully parsed: {:#?}", &response);
    Ok(())
}