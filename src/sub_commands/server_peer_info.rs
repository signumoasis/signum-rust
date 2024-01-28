
#[derive(serde::Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMyPeerInfoResponse {
    pub uts_in_store: i32,
    pub request_processing_time: i32,
}

pub async fn handle_serverinfo_getmypeerinfo( 
    address: &str,
) -> Result<GetMyPeerInfoResponse, std::io::Error> {
    let response = reqwest::Client::new()
        .get(&format!("{}/burst?requestType=getMyPeerInfo", address))
        .send()
        .await
        .expect("Failed to execute request.");

    Ok(response.json::<GetMyPeerInfoResponse>().await.unwrap())
}
