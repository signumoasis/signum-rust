use clap::SubCommand;
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

pub fn add_subcommand_server_info<'a, 'b>(app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
    app.subcommand(
        SubCommand::with_name("getmyinfo")
            .about("Displays information about this server.")
            .version("0.1.0"),
    )
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
