use clap::SubCommand;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetPeersResponse {
    pub peers: Vec<std::net::IpAddr>,
    pub request_processing_time: i32,
}

pub fn subcommand_get_peers<'a, 'b>(app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
    app.subcommand(
        SubCommand::with_name("getpeers")
            .about("Lists this server's peers.")
            .version("0.1.0"),
    )
}

pub async fn handle_get_peers(address: &str) -> Result<GetPeersResponse, std::io::Error> {
    let response = reqwest::Client::new()
        .get(&format!("{}/burst?requestType=getPeers", address))
        .send()
        .await
        .expect("Failed to execute request.");

    Ok(response.json::<GetPeersResponse>().await.unwrap())
}
