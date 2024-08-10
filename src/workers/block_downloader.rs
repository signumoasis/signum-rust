use anyhow::Result;
use num_bigint::BigUint;
use std::{collections::VecDeque, time::Duration};
use tokio::task::{JoinHandle, JoinSet};
use tracing::{instrument, Instrument};
use uuid::Uuid;

use crate::{
    configuration::Settings,
    models::{
        datastore::Datastore,
        p2p::{B1Block, PeerAddress},
    },
    peers::{B1Peer, BasicPeerClient, DownloadResult},
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

#[derive(Debug)]
struct DownloadJob {
    start_height: u64,
    number_of_blocks: u32,
    peer: PeerAddress,
    retries: u32,
}

/// This worker queries random peers for new blocks.
#[tracing::instrument(name = "Block Downloader", skip_all)]
pub async fn block_downloader(mut database: Datastore, _settings: Settings) -> Result<()> {
    tracing::info!("Would download blocks");
    // Steps:
    // * Initiate FIFO queue
    // * Determine highest block stored in DB
    // * Get a few random peers, up to 15
    // * Determine the mode of the highest cumulative difficulty
    // of a subset of known nodes, with a maximum of 15 or something
    // * Store highest cumulative difficulty.
    // * Filter the selection of peers to remove any that don't match the target CD
    // * Loop through peers, spawning download tasks until we reach the
    // download instances limit
    // * Store the download task in the VedDeque
    // * Loop
    //      * pop first task
    //      * check results for errors, requeue if error; consider dropping entire queue
    //      and recreating it to dump now-defunct follow-on tasks
    //      * push new download tasks, if needed and slots are available
    //
    // NOTES:
    // Things to keep track of:
    // * Which ones have errored - handled by FIFO task queue, but must be re-requested
    //
    let mut downloads = VecDeque::<JoinHandle<_>>::new();
    let max_download_tasks = 8;

    //let mut highest_cumulative_difficulty = BigUint::ZERO;
    let peers = database.get_n_random_peers(15).await?;

    tracing::debug!("Random peers from db: {:#?}", &peers);

    let mut cumulative_difficulties: Vec<(PeerAddress, BigUint)> = Vec::new();

    let mut joinset = JoinSet::new();
    for peer_address in peers {
        tracing::trace!("Queueing get cumulative difficulty from {}.", &peer_address);
        let peer = B1Peer::new(peer_address.clone());
        joinset.spawn(async move {
            let cd = peer.get_peer_cumulative_difficulty().await?;
            Ok::<(PeerAddress, BigUint), anyhow::Error>((peer_address, cd))
        });
    }
    while let Some(joinhandle) = joinset.join_next().await {
        //tracing::trace!("Getting cumulative difficulty from {}.", &peer);
        // Get cumulative difficulties to find the most common one
        match joinhandle {
            Ok(result) => match result {
                Ok(r) => {
                    cumulative_difficulties.push(r);
                }
                Err(e) => {
                    tracing::warn!(
                        "Unable to get cumulative difficulty from one of the peers.\n\tCaused by: {}",
                        e
                    );
                }
            },
            Err(e) => {
                tracing::error!("Error caused by:\n\t{}", e);
            }
        }
    }

    tracing::debug!("Cumulative difficulties: {:?}", cumulative_difficulties);

    let highest_cumulative_difficulty = statistics_mode(
        cumulative_difficulties
            .iter()
            .map(|(_peer, cd)| cd)
            .collect::<Vec<_>>(),
    )
    .unwrap_or(&BigUint::ZERO)
    .to_owned();

    let download_peers = cumulative_difficulties
        .into_iter()
        .filter(|(_peer, cd)| *cd == highest_cumulative_difficulty)
        .map(|(peer, _cd)| peer)
        .collect::<Vec<_>>();

    tracing::debug!(
        "Highest cumulative difficulty: {}",
        highest_cumulative_difficulty
    );

    //TODO: Do this in a loop, setting up appropriate sets of blocks
    let mut queued_download_tasks = 1;
    for peer in download_peers {
        tracing::trace!("Queueing {} for block download.", &peer);
        let job = DownloadJob {
            peer,
            start_height: 10 * queued_download_tasks,
            number_of_blocks: 10,
            retries: 0,
        };
        downloads.push_back(tokio::spawn(download_blocks_task(job)));
        queued_download_tasks += 1;
        if queued_download_tasks > max_download_tasks {
            break;
        }
        //TODO: Skip peer; it's probably bad. Consider blacklisting if main node does
    }

    // Loop over jobs, in order, stitching if they're correct
    tracing::trace!("{} DOWNLOAD TASKS QUEUED", downloads.len());
    tracing::trace!("{:#?}", &downloads);
    while let Some(download_task) = downloads.pop_front() {
        match download_task.await? {
            Ok(result) => {
                tracing::trace!(
                    "QUEUE BLOCK PROCESSING: {} - height: {} - number_of_blocks: {}",
                    result.peer,
                    result.start_height,
                    result.number_of_blocks
                );
                tracing::trace!("Blocks remaining in queue: {}", downloads.len());
            }
            Err(mut e) => {
                //Requeue job, push to front as we're popping them from the front
                //anyways and we would like them to stay in order

                //first check retries and cancel everything if it's failed N times
                if e.retries > 3 {
                    //explode
                }
                e.retries += 1;
                downloads.push_front(tokio::spawn(download_blocks_task(e)));
            }
        }
    }

    Ok(())
}

//TODO: Rework the output of this to use a custom error type that includes the reason and DownloadJob
#[instrument(name = "Download Blocks Task")]
async fn download_blocks_task(job: DownloadJob) -> Result<DownloadResult, DownloadJob> {
    tracing::trace!(
        "Downloading blocks {} through {} from {}.",
        &job.start_height,
        &job.number_of_blocks,
        &job.peer
    );

    let peer = B1Peer::new(job.peer.clone());

    let result = match peer
        .get_blocks_from_height(job.start_height, job.number_of_blocks)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            tracing::error!(
                "Unable to get blocks from {}.\n\tCaused by: {}",
                &job.peer,
                e
            );
            return Err(job);
        }
    };

    //TODO: Process the blocks just downloaded and return the correct Result
    // - OK if all in this subchain are good
    // - Connection error for any connectivity issues
    // - Parse or Verification error for bad blocks
    Ok(result)
}

async fn _verify_b1_block(_block: B1Block) -> anyhow::Result<B1Block> {
    todo!()
}
