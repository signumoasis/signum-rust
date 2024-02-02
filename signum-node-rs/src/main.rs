use std::{
    fmt::{Debug, Display},
    str::FromStr,
    time::Duration,
};

use anyhow::{Context, Result};

//use signum_node_rs::peer_service::{run_peer_service, Peer, PeerContainer, PeerServiceHandle};
use signum_node_rs::{
    get_peer_info, get_peers,
    models::p2p::{PeerAddress, PeerInfo},
    peer_service::PeerServiceHandle,
    telemetry::{get_subscriber, init_subscriber},
};
use tokio::{
    task::{JoinError, JoinHandle},
    time,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Begin by setting up tracing
    let subscriber = get_subscriber("signum-node-rs".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    start().await
}

#[tracing::instrument]
async fn start() -> Result<()> {
    let interval_task = tokio::spawn(interval_actor_demo());
    // let peer_task = tokio::spawn(run_peer_demo());
    let peer_task = tokio::spawn(get_peers_task());

    tokio::select! {
        o = interval_task => report_exit("Interval Task", o),
        o = peer_task => report_exit("Peer Task", o),
    };

    // let addy = "http://p2p.signumoasis.xyz:80".parse::<PeerAddress>()?;

    // tracing::debug!(address=?addy,"SIGNIFICANT EMOTIONAL EVENT");

    // DON'T DO MORE STUFF
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

/// Gets a node's peers, then loops them all and gets a PeerInfo for each one
///
/// Example of a possible way to handle peer management.
/// There would likely be several worker tasks set up. This one would be split in two:
/// One to get all the peers for a peer
///     - checks if a peer exists in db and only writes if its new
/// Second to get a peerinfo for each peer that doesn't have it set or it's older than x seconds
///     - checks last update time, only updating if null or too old
///     - writes result to db if successful, otherwise just quits with error logged
/// These could possibly both be child tasks of a manager task
///
/// Need to figure out how to respawn manager task instead of quitting program after
/// a single tasks exits
async fn get_peers_task() -> Result<()> {
    let addy = PeerAddress::from_str("http://p2p.signumoasis.xyz:80")
        .context("Couldn't parse peer address")?;
    tracing::debug!("Downloading peers from {}", addy);
    let r = get_peers(addy).await?;
    tracing::debug!("Peers downloaded: {:#?}", r);
    let mut tasks = Vec::new();

    for peer in r.into_iter() {
        tasks.push(tokio::spawn(get_peer_info(peer)));
    }

    // handles not necessary in final task. Fire and forget. Each task can write its update to db
    // directly instead of going through a manager. db is source of truth and cache
    let mut results = Vec::with_capacity(tasks.len());
    for handle in tasks {
        results.push(handle.await.unwrap());
    }

    tracing::debug!("{:#?}", results);

    Ok(())
}

async fn interval_actor_demo() -> Result<()> {
    let _peer = PeerServiceHandle::new();

    let mut interval = time::interval(time::Duration::from_secs(1));
    for _i in 0..10 {
        tracing::debug!("Interval Tick");
        interval.tick().await;
    }

    Ok(())
}

async fn run_peer_demo() -> Result<()> {
    use std::collections::HashMap;

    let address = "http://p2p.signumoasis.xyz";

    let mut thebody = HashMap::new();
    thebody.insert("protocol", "B1");
    thebody.insert("requestType", "getPeers");

    let peer_request = reqwest::Client::new()
        .post(address)
        .header("User-Agent", "BRS/3.8.0")
        .json(&thebody)
        .send()
        .await?;

    tracing::debug!("Parsing peers");
    #[derive(Debug, serde::Deserialize)]
    struct PeerContainer {
        #[serde(rename = "peers")]
        _peers: Vec<PeerAddress>,
    }
    let peers = peer_request.json::<PeerContainer>().await?;
    tracing::debug!("{:#?}", &peers);

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    // Ok(())
}
