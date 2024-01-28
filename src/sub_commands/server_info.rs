use uuid::Uuid;

#[derive(serde::Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMyInfoResponse {
    pub host: String,
    pub address: String,
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    pub request_processing_time: i32,
}

pub async fn handle_serverinfo_getmyinfo(
    address: &str,
) -> Result<GetMyInfoResponse, std::io::Error> {
    let response = reqwest::Client::new()
        .get(&format!("{}/burst?requestType=getMyInfo", address))
        .send()
        .await
        .expect("Failed to execute request.");

    Ok(response.json::<GetMyInfoResponse>().await.unwrap())
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetPeersResponse {
    pub peers: Vec<std::net::IpAddr>,
    pub request_processing_time: i32,
}

pub async fn handle_get_peers(address: &str) -> Result<GetPeersResponse, std::io::Error> {
    let response = reqwest::Client::new()
        .get(&format!("{}/burst?requestType=getPeers", address))
        .send()
        .await
        .expect("Failed to execute request.");

    Ok(response.json::<GetPeersResponse>().await.unwrap())
}
