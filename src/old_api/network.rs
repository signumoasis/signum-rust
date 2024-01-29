use anyhow::Context;
use uuid::Uuid;

pub async fn get_my_info(address: &str) -> Result<GetMyInfoResponse, anyhow::Error> {
    let response = reqwest::Client::new()
        .get(&format!("{}/api?requestType=getMyInfo", address))
        .send()
        .await
        .context("Failed to execute request.")?;

    response
        .json::<GetMyInfoResponse>()
        .await
        .context("Unable to parse response")
}

/// `active` peers are all peers _not_ listed as NOT_CONNECTED
/// so it includes CONNECTED _and_ DISCONNECTED states.
/// `active` overrides the state selection if true, and is ignored if false
/// or not present, therefore `active` and `state` are mutually exclusive options.
pub async fn get_peers(
    address: &str,
    active: &bool,
    state: &PeerStates,
) -> Result<GetPeersResponse, anyhow::Error> {
    let mut request_url = format!("{}/api?requestType=getPeers", address);

    if *active {
        request_url = format!("{}&active=true", request_url);
    }

    request_url = format!(
        "{}{}",
        request_url,
        match state {
            PeerStates::All => "",
            PeerStates::Connected => "&state=CONNECTED",
            PeerStates::Disconnected => "&state=DISCONNECTED",
            PeerStates::NonConnected => "&state=NON_CONNECTED",
        }
    );

    let response = reqwest::Client::new()
        .get(request_url)
        .send()
        .await
        .context("Failed to execute request.")?;

    response
        .json::<GetPeersResponse>()
        .await
        .context("Unable to parse response")
}

pub async fn get_my_peer_info(address: &str) -> Result<GetMyPeerInfoResponse, anyhow::Error> {
    let response = reqwest::Client::new()
        .get(format!("{}/api?requestType=getMyPeerInfo", address))
        .send()
        .await
        .context("Failed to execute request")?;

    response
        .json::<GetMyPeerInfoResponse>()
        .await
        .context("Unable to parse response")
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum GetMyInfoResponse {
    #[serde(rename_all = "camelCase")]
    Response {
        host: String,
        address: String,
        #[serde(rename = "UUID")]
        uuid: Uuid,
        request_processing_time: i32,
    },
    #[serde(rename_all = "camelCase")]
    Error {
        error_code: i32,
        error_description: String,
        request_processing_time: Option<i32>,
    },
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum GetPeersResponse {
    #[serde(rename_all = "camelCase")]
    Response {
        peers: Vec<std::net::IpAddr>,
        request_processing_time: i32,
    },
    #[serde(rename_all = "camelCase")]
    Error {
        error_code: i32,
        error_description: String,
        request_processing_time: Option<i32>,
    },
}

#[derive(Clone, Debug)]
pub enum PeerStates {
    All,
    Connected,
    Disconnected,
    NonConnected,
}


#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum GetMyPeerInfoResponse {
    #[serde(rename_all = "camelCase")]
    Response {
        uts_in_store: i32,
        request_processing_time: i32,
    },
    #[serde(rename_all = "camelCase")]
    Error {
        error_code: i32,
        error_description: String,
        request_processing_time: Option<i32>,
    },
}
