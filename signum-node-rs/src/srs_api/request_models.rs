use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
#[serde(tag = "requestType")]
pub enum RequestType {
    AddPeers {
        protocol: String,
        peers: Vec<String>,
    },
    GetPeers {
        protocol: String,
    },
}
