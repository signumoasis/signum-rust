
use uuid::Uuid;

pub async fn serverinfo_getmyinfo(address: &str) -> Result<GetMyInfoResponse, std::io::Error> {
    let response = reqwest::Client::new()
        .get(&format!("{}/burst?requestType=getMyInfo", address))
        .send()
        .await
        .expect("Failed to execute request.");

    Ok(response.json::<GetMyInfoResponse>().await.unwrap())
}

#[derive(serde::Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMyInfoResponse {
    pub host: String,
    pub address: String,
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    pub request_processing_time: i32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
