use std::{
    fmt::{Debug, Display},
    time::Duration,
};

use anyhow::Result;

//use signum_node_rs::peer_service::{run_peer_service, Peer, PeerContainer, PeerServiceHandle};
use signum_node_rs::{
    models::p2p::PeerAddress,
    peer_service::PeerServiceHandle,
    telemetry::{get_subscriber, init_subscriber},
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
    let interval_task = tokio::spawn(interval_actor_demo());
    let peer_task = tokio::spawn(run_peer_demo());

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
    Ok(())
}
