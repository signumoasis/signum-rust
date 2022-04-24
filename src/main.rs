use anyhow::Result;
use std::collections::HashMap;

use signum_node_rs::peer_service::{Peer, PeerContainer};

#[tokio::main]
async fn main() -> Result<()> {
    let address = "http://p2p.signumoasis.xyz";

    let mut thebody = HashMap::new();
    thebody.insert("protocol", "B1");
    thebody.insert("requestType", "getPeers");

    let peer_request = reqwest::Client::new()
        .post(address)
        .header("User-Agent", "BRS/3.3.4")
        .json(&thebody)
        .send()
        .await?;

    let peers = peer_request.json::<PeerContainer>().await?.peers;

    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    for p in peers {
        let tx = tx.clone();
        // Get each peer's peerinfo
        tokio::spawn(async move {
            // let p = PeerAddress("p2p.signumoasis.xyz".to_string());
            let mut peer_req_body = HashMap::new();
            peer_req_body.insert("protocol", "B1");
            peer_req_body.insert("requestType", "getInfo");

            let port = if p.0.split(':').count() >= 2 {
                ""
            } else {
                ":8123"
            };
            let addy = format!("http://{}{}", p.0.clone(), port);

            println!("DEBUG: Checking `{}`", p.0.clone());
            let info = reqwest::Client::new()
                .post(addy)
                .header("User-Agent", "BRS/3.3.4")
                .json(&peer_req_body)
                .send()
                .await;

            println!("DEBUG: Received from `{}`. Deserializing", p.0.clone());
            let peer = match info {
                Ok(r) => match r.json::<Peer>().await {
                    Ok(r) => r,
                    Err(e) => {
                        println!("WARNING: Bad peer: {:#?}\n\nError: {}", p.0, e);
                        panic!("IT BROKE!")
                    }
                },
                Err(e) => {
                    println!("WARNING: Bad peer: {:#?}\n\nError: {}", p.0, e);
                    panic!("IT BROKE!")
                }
            };
            println!(
                "DEBUG: Deserialized `{}`. Sending answer to main thread",
                p.0.clone()
            );
            tx.send(peer).await
        });
    }
    drop(tx);

    let mut all_peers = Vec::new();
    while let Some(message) = rx.recv().await {
        all_peers.push(message);
    }

    dbg!(all_peers);
    Ok(())
}
