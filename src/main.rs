use anyhow::Result;

//use signum_node_rs::peer_service::{run_peer_service, Peer, PeerContainer, PeerServiceHandle};
use signum_node_rs::{
    models::p2p::PeerAddress,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
#[tracing::instrument(name = "Main")]
async fn main() -> Result<()> {
    // Begin by setting up tracing
    let subscriber = get_subscriber("signum-node-rs".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    tracing::info!("Started the program.");

    // DO STUFF BELOW HERE

    let addy = "http://p2p.signumoasis.xyz:80".parse::<PeerAddress>()?;

    dbg!(addy);


    // DON'T DO MORE STUFF
    Ok(())
}

// async fn interval_actor_demo() {
//     let peer = PeerServiceHandle::new();

//     let mut interval = time::interval(time::Duration::from_secs(5));
//     for _i in 0..30 {
//         interval.tick().await;
//     }
// }

// async fn run_peer_demo() -> Result<()> {
//     let address = "http://p2p.signumoasis.xyz";

//     let mut thebody = HashMap::new();
//     thebody.insert("protocol", "B1");
//     thebody.insert("requestType", "getPeers");

//     let peer_request = reqwest::Client::new()
//         .post(address)
//         .header("User-Agent", "BRS/3.3.4")
//         .json(&thebody)
//         .send()
//         .await?;

//     let peers = peer_request.json::<PeerContainer>().await?.peers;

//     let (tx, mut rx) = tokio::sync::mpsc::channel(100);

//     for p in peers {
//         let tx = tx.clone();
//         // Get each peer's peerinfo
//         //TODO: move this getInfo call into the Peer impl on a function
//         tokio::spawn(async move {
//             // let p = PeerAddress("p2p.signumoasis.xyz".to_string());
//             let mut peer_req_body = HashMap::new();
//             peer_req_body.insert("protocol", "B1");
//             peer_req_body.insert("requestType", "getInfo");

//             let port = if p.0.split(':').count() >= 2 {
//                 ""
//             } else {
//                 ":8123"
//             };
//             let addy = format!("http://{}{}", p.0.clone(), port);

//             tracing::debug!("Checking `{}`", p.0.clone());
//             let info = reqwest::Client::new()
//                 .post(addy)
//                 .header("User-Agent", "BRS/3.3.4")
//                 .json(&peer_req_body)
//                 .send()
//                 .await;

//             tracing::debug!("Received from `{}`. Deserializing", p.0.clone());
//             let peer = match info {
//                 Ok(r) => match r.json::<Peer>().await {
//                     Ok(r) => r,
//                     Err(e) => {
//                         tracing::warn!("WARNING: Bad peer: {:#?}\n\nError: {}", p.0, e);
//                         panic!("IT BROKE!")
//                     }
//                 },
//                 Err(e) => {
//                     tracing::warn!("WARNING: Bad peer: {:#?}\n\nError: {}", p.0, e);
//                     panic!("IT BROKE!")
//                 }
//             };
//             tracing::debug!(
//                 "Deserialized `{}`. Sending answer to main thread",
//                 p.0.clone()
//             );
//             tx.send(peer).await
//         });
//     }
//     drop(tx);

//     let mut all_peers = Vec::new();
//     while let Some(message) = rx.recv().await {
//         all_peers.push(message);
//     }

//     dbg!(all_peers);
//     Ok(())
// }
