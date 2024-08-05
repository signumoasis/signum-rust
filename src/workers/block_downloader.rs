use anyhow::{Context, Result};
use futures::stream::FuturesOrdered;
use futures::stream::Stream;
use num_bigint::BigUint;
use serde_json::json;
use std::{cmp::Ordering, time::Duration};
use tracing::{instrument, Instrument};
use uuid::Uuid;

use crate::{
    configuration::Settings,
    models::{
        datastore::Datastore,
        p2p::{B1Block, PeerAddress},
    },
    peers::get_peer_cumulative_difficulty,
    statistics_mode,
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
    // of several or maybe all known peers
    // * Store highest cumulative difficulty.
    // * Establish a FuturesOrdered
    // * Loop through random peers, triggering download tasks if that peer matches the
    // cumulative difficulty we have just stored, or higher, until we read the
    // download instances limit
    // * Store the download task in the FuturesOrdered
    // * Loop
    //      * call poll_next on the FuturesOrdered
    //      * pop results
    //      * check results for errors, requeue if error; consider dropping entire FuturesOrdered
    //      and recreating it to dump now-defunct follow-on tasks
    //      * push new download tasks, if needed and slots are available
    //
    // NOTES:
    // Things to keep track of:
    // * Which block sets have been queried - handled by FIFO task queue
    // * Which ones have errored - handled by FIFO task queue, but must be re-requested
    //
    let mut downloads = FuturesOrdered::new();
    let max_download_tasks = 8;

    //let mut highest_cumulative_difficulty = BigUint::ZERO;
    let peers = database.get_n_random_peers(10).await?;

    tracing::debug!("Random peers from db: {:#?}", &peers);

    let mut cumulative_difficulties: Vec<BigUint> = Vec::new();

    for peer in peers {
        // Get cumulative difficulties to find the most common one
        let cd = get_peer_cumulative_difficulty(peer).await?;
        cumulative_difficulties.push(cd);
    }

    tracing::debug!(
        "Highest cumulative difficulties: {:#?}",
        cumulative_difficulties
    );

    let highest_cumulative_difficulty =
        statistics_mode(cumulative_difficulties).unwrap_or(BigUint::ZERO);
    tracing::debug!(
        "Highest cumulative difficulty: {}",
        highest_cumulative_difficulty
    );

    //TODO: Do this in a loop, setting up appropriate sets of blocks
    let mut queued_download_tasks = 0;
    loop {
        let peer = database
            .get_random_peer()
            .await
            .context("couldn't get random peer from database")?;
        let peer_cumulative_difficulty = get_peer_cumulative_difficulty(peer.clone())
            .await
            .context("unable to get peer cumulative difficulty")?;
        if peer_cumulative_difficulty == highest_cumulative_difficulty {
            tracing::trace!("Queueing {} for block download.", &peer);
            downloads.push_back(download_blocks_task(peer, 1, 10));
            queued_download_tasks += 1;
        } else {
            let comparison = match peer_cumulative_difficulty.cmp(&highest_cumulative_difficulty) {
                Ordering::Less => "less than",
                Ordering::Greater => "greater than",
                Ordering::Equal => "the same as", // This one should actually never happen
            };

            tracing::warn!(
                "Not downloading from {} because cumulative difficulty ({}) is {}
                the target ({}).",
                &peer,
                &peer_cumulative_difficulty,
                comparison,
                &highest_cumulative_difficulty,
            );
        }
        if queued_download_tasks >= max_download_tasks {
            break;
        }
        //TODO: Skip peer; it's probably bad. Consider blacklisting if main node does

        // DISABLED - Unsure we need to do this
        //if highest_cumulative_difficulty < peer_cumulative_difficulty {
        //    highest_cumulative_difficulty = peer_cumulative_difficulty;
        //}
    }

    // Loop over jobs, polling if they're done or not
    tracing::trace!("Need to loop download jobs now and wait for them to finish");
    loop {
        let x = downloads.poll_next();
    }

    Ok(())
}

#[instrument(name = "Download Blocks Task")]
async fn download_blocks_task(peer: PeerAddress, height: u64, number_of_blocks: u64) -> Result<()> {
    let _thebody = json!({
        "protocol": "B1",
        "requestType": "getBlocksFromHeight",
        "height": height,
        "numBlocks": number_of_blocks,
    });

    tracing::trace!(
        "Downloading blocks {} through {} from {}.",
        &height,
        &number_of_blocks,
        &peer
    );

    //TODO: Process the blocks just downloaded and return the correct Result
    // - OK if all in this subchain are good
    // - Connection error for any connectivity issues
    // - Parse or Verification error for bad blocks
    Ok(())
}

async fn _verify_b1_block(_block: B1Block) -> anyhow::Result<B1Block> {
    todo!()
}
