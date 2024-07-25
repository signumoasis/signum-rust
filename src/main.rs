use std::fmt::{Debug, Display};

use anyhow::Result;

use signum_node_rs::{
    configuration::get_configuration,
    srs_api::SrsApiApplication,
    telemetry::{get_subscriber, init_subscriber},
    workers::{
        block_downloader::run_block_downloader_forever, peer_finder::run_peer_finder_forever,
        peer_info_trader::run_peer_info_trader_forever,
    },
};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> Result<()> {
    // Begin by setting up tracing
    let subscriber = get_subscriber("signum-node-rs".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    start().await
}

//pub static CONFIGURATION: Settings = configuration::get_configuration();

#[tracing::instrument]
async fn start() -> Result<()> {
    let configuration =
        get_configuration().expect("Couldn't get the configuration. Unable to continue");

    //let configuration = CONFIGURATION;

    let database = configuration.database.get_db().await?;

    // Create the Block Downloader task
    let block_downloader_task = tokio::spawn(run_block_downloader_forever(
        database.clone(),
        configuration.clone(),
    ));

    // Create the p2p api webserver task
    let p2p_api = SrsApiApplication::build(configuration.clone(), database.clone()).await?;
    let p2p_api_task = tokio::spawn(p2p_api.run_until_stopped());

    // Create the peer finder task
    let peer_finder_task = tokio::spawn(run_peer_finder_forever(database.clone(), configuration));

    // Create the peer info trader task
    let peer_info_trader_task = tokio::spawn(run_peer_info_trader_forever(database));

    // Start all the tasks
    tokio::select! {
        o = block_downloader_task=> report_exit("Block Downloader", o),
        o = p2p_api_task => report_exit("P2P API Server", o),
        o = peer_finder_task => report_exit("Peer Finder", o),
        o = peer_info_trader_task => report_exit("Peer Info Trader", o),
    };

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} task failed to complete",
                task_name
            )
        }
    }
}
