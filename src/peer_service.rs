use std::{collections::HashMap, sync::Arc, time::Duration};

use rand::prelude::IteratorRandom;
use serde::Deserialize;
use tokio::{
    sync::{mpsc, oneshot},
    time,
};

use crate::{
    models::p2p::{PeerAddress, PeerInfo},
    peer::PeerHandle,
};

pub const USER_AGENT: &str = "BRS/3.3.4";

/// The PeerService discovers, manages, and blacklists peers.
/// It also returns peers to be used by other services.
struct PeerService {
    receiver: mpsc::Receiver<PeerServiceMessage>,
    peers_cache: HashMap<PeerAddress, PeerHandle>,
    blacklisted_peer_cache: HashMap<PeerAddress, PeerHandle>,
}
impl PeerService {
    #[tracing::instrument(name = "PeerService.new()", skip(receiver))]
    pub fn new(receiver: mpsc::Receiver<PeerServiceMessage>) -> Self {
        //TODO: Get seed peers from config and load peers from database
        // simulate seed peer
        // let seed_peer = Peer {
        //     ip_address: "".into(),
        //     port: 80,
        //     address: Some(PeerAddress("p2p.signumoasis.xyz:80".into())),
        //     application: "".into(),
        //     version: "".into(),
        //     platform: Some("".into()),
        //     share_address: true,
        // };

        let mut initial_cache = HashMap::new();
        // if let Some(peer_address) = &seed_peer.address {
        //     initial_cache.insert(peer_address.clone(), seed_peer);
        // }
        Self {
            receiver,
            peers_cache: initial_cache,
            blacklisted_peer_cache: HashMap::<PeerAddress, PeerHandle>::new(),
        }
    }

    #[tracing::instrument(name = "PeerService.handle_message()", skip(self))]
    fn handle_message(&mut self, msg: PeerServiceMessage) {
        match msg {
            PeerServiceMessage::AddPeer { respond_to } => todo!(),
            PeerServiceMessage::GetRandomPeer { respond_to } => {
                //TODO: Consider randomly selecting from the database?
                match self.peers_cache.keys().choose(&mut rand::thread_rng()) {
                    Some(key) => {
                        let _ = respond_to.send(self.peers_cache.get(key).cloned());
                    }
                    None => {
                        let _ = respond_to.send(None);
                    }
                }
            }
            PeerServiceMessage::GetPeer {
                respond_to,
                peer_address,
            } => {
                let peer = self.peers_cache.get(&peer_address);
                let _ = respond_to.send(peer.cloned());
            }
            PeerServiceMessage::BlacklistPeer { respond_to } => {
                let _ = respond_to.send(Ok(()));
            }
        }
    }

    #[tracing::instrument(name = "PeerService.discover_peers()", skip(self))]
    pub(crate) async fn discover_peers(&mut self) {
        // 1. Get a clone of all PeerHandles in cache/datastore
        // 2. Spin off tokio spawn to Get each peers' peers and return results here
        // 3. Spin up new peer actors for each PeerAddress and add to local collection of PeerHandles
        // 4. Loop all peers in local collection to spin up new tokio::spawn get peer info and return PeerInfo results here
        // 5. On success, report any PeerInfos to the PeerService, which will add them to datastore

        let local_cache = self.peers_cache.clone();
        for p in local_cache.into_values() {}

        // tracing::debug!("PEERS CACHE - PRE-DISCOVERY\n{:?}", &self.peers_cache);
        // for (_, p) in self.peers_cache.clone().iter_mut() {
        //     let refreshed_peer = match p.call_get_info().await {
        //         Ok(x) => x,
        //         Err(e) => {
        //             tracing::error!("error with peer discovery: {}", e);
        //             return;
        //         }
        //     };

        //     if let Some(peer_address) = &refreshed_peer.address {
        //         self.peers_cache
        //             .insert(peer_address.clone(), refreshed_peer);
        //     }
        // }
        // tracing::debug!("PEERS CACHE - POST-DISCOVERY\n{:?}", &self.peers_cache);
    }
}

/// A handle to interact with the [`PeerService`].
#[derive(Clone)]
pub struct PeerServiceHandle {
    sender: mpsc::Sender<PeerServiceMessage>,
}
impl PeerServiceHandle {
    #[tracing::instrument(name = "PeerServiceHandle.new()")]
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = PeerService::new(receiver);
        tokio::spawn(run_peer_service(actor));

        Self { sender }
    }

    /// Requests a random `Option<[Peer]>` from the [`PeerService`]. Returns None if
    /// no peer can be found.
    #[tracing::instrument(name = "PeerServiceHandle.get_random_peer()", skip(self))]
    pub async fn get_random_peer(&self) -> Option<PeerHandle> {
        let (tx, rx) = oneshot::channel();
        let msg = PeerServiceMessage::GetRandomPeer { respond_to: tx };

        let _ = self.sender.send(msg).await;
        rx.await.expect("PeerService has been killed")
    }

    /// Requests a specific `Option<[Peer]>` by `[PeerAddress]` from the [`PeerService`]. Returns None if
    /// no peer can be found.
    #[tracing::instrument(name = "PeerServiceHandle.get_peer_by_address()", skip(self))]
    pub async fn get_peer_by_address(&self, address: PeerAddress) -> Option<PeerHandle> {
        let (tx, rx) = oneshot::channel();
        let msg = PeerServiceMessage::GetPeer {
            respond_to: tx,
            peer_address: address,
        };

        let _ = self.sender.send(msg).await;
        rx.await.expect("PeerService has been killed")
    }

    pub async fn test_event(&self) -> String {
        let (tx, rx) = oneshot::channel();
        let msg = PeerServiceMessage::BlacklistPeer { respond_to: tx };
        let _ = self.sender.send(msg).await;

        match rx.await {
            Ok(_) => "Successful blacklist test.".into(),
            Err(_) => "F- on your test".into(),
        }
    }
}

#[tracing::instrument(name = "run_peer_service", skip(service))]
async fn run_peer_service(mut service: PeerService) {
    let mut interval = time::interval(Duration::from_millis(30000));
    loop {
        tokio::select! {
            // PeerServiceMessage handler
            Some(peer_msg) = service.receiver.recv() => service.handle_message(peer_msg),
            // Peer discovery task
            _ = interval.tick() => service.discover_peers().await,
        }
    }
}

#[derive(Debug)]
pub enum PeerServiceMessage {
    AddPeer {
        respond_to: oneshot::Sender<Option<PeerInfo>>,
    },
    GetRandomPeer {
        respond_to: oneshot::Sender<Option<PeerHandle>>,
    },
    GetPeer {
        respond_to: oneshot::Sender<Option<PeerHandle>>,
        peer_address: PeerAddress,
    },
    BlacklistPeer {
        respond_to: oneshot::Sender<anyhow::Result<()>>,
    },
}

// /// Necessary because of poor json formatting decisions in the source data.
// #[derive(Debug, Deserialize)]
// pub struct PeerContainer {
//     pub peers: Vec<PeerAddress>,
// }

// TODO: This peer object needs to be separated into two
// One for the json deserialization and one for internal use
// because the json doesn't include the actual IP and port
// but this app needs to serialize and deserialize them
// #[derive(Clone, Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Peer {
//     p2p_address: Option<PeerAddress>,
//     public_api_address: Option<String>,
//     application: String,
//     version: String,
//     platform: Option<String>,
//     share_address: bool,
// }
// impl Peer {
//     pub async fn get_peers() {}

//     #[tracing::instrument(name = "Peer.get_info()", skip(self))]
//     pub(crate) async fn get_info(&self) -> Result<Peer, anyhow::Error> {
//         tracing::debug!("Started to get info");
//         let (request, mut body) = self.get_request_p2p_client().await?;
//         body.insert("requestType".into(), "getInfo".into());

//         let response = request.json(&body).send().await?;

//         tracing::debug!("Received from `{}`. Deserializing", &self.ip_address);

//         let peer = response.json::<Peer>().await?;

//         tracing::debug!(
//             "Deserialized `{}`. Sending answer to main thread",
//             &self.ip_address
//         );
//         Ok(peer)
//     }

//     async fn get_request_p2p_client(
//         &self,
//     ) -> Result<(reqwest::RequestBuilder, HashMap<String, String>), anyhow::Error> {
//         let ip = match &self.address {
//             Some(announced_address) => {
//                 match tokio::net::lookup_host(announced_address.0.to_string()).await {
//                     Ok(_) => announced_address.0.clone(),
//                     Err(e) => {
//                         return Err(anyhow::anyhow!(
//                             "error getting socket address from announced address: {}",
//                             e
//                         ));
//                     }
//                 }
//             }
//             None => match tokio::net::lookup_host(&self.ip_address).await {
//                 Ok(ips) => ips.collect::<Vec<_>>()[0].to_string(),
//                 Err(e) => {
//                     return Err(anyhow::anyhow!("ip address could not be turned into socket address and no announced address present: {}",e));
//                 }
//             },
//         };

//         let url = format!("http://{}", ip);
//         tracing::debug!("Signum P2P Url Requested: {}", &url);
//         let builder = reqwest::Client::new()
//             .post(url)
//             .header("User-Agent", USER_AGENT);
//         let mut body = HashMap::<String, String>::new();
//         body.insert("protocol".into(), "B1".into());
//         Ok((builder, body))
//     }
// }
