use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::{Context, Result};

//use signum_node_rs::peer_service::{run_peer_service, Peer, PeerContainer, PeerServiceHandle};
use signum_node_rs::{
    configuration::get_configuration,
    get_peer_info, get_peers,
    models::p2p::PeerAddress,
    telemetry::{get_subscriber, init_subscriber},
    workers::peer_finder::run_peer_finder_forever,
};
use tokio::{task::JoinError, time};

#[tokio::main]
async fn main() -> Result<()> {
    // Begin by setting up tracing
    let subscriber = get_subscriber("signum-node-rs".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    start().await
}

#[tracing::instrument]
async fn start() -> Result<()> {
    let configuration =
        get_configuration().expect("Couldn't get the configuration. Unable to continue");
    let interval_task = tokio::spawn(interval_actor_demo());
    // let peer_task = tokio::spawn(get_peers_task());
    let peer_finder_task = tokio::spawn(run_peer_finder_forever(configuration));

    tokio::select! {
        o = peer_finder_task => report_exit("Peer Finder", o),
        o = interval_task => report_exit("Interval Task", o),
        // o = peer_task => report_exit("Peer Task", o),
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

async fn interval_actor_demo() -> Result<()> {
    let mut interval = time::interval(time::Duration::from_secs(10));
    for _ in 1..30 {
        tracing::debug!("Interval Tick");
        interval.tick().await;
    }
    Ok(())
}
