// use clap::SubCommand;
// use serde::{Deserialize, Serialize};

// pub const SUBCOMMAND_NAME: &str = "ping";
// pub trait ClapAppPingExtension {
//     fn add_ping_subcommands(self) -> Self;
// }
// impl<'a, 'b> ClapAppPingExtension for clap::App<'a, 'b> {
//     fn add_ping_subcommands(self) -> Self {
//         let scmd = SubCommand::with_name("ping")
//             .about("Attempts to get blocks from a node's public API")
//             .version("0.1.0");
//         self.subcommand(scmd)
//     }
// }

// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// pub struct PingResponse {
//     number_pings_sent: u64,
//     number_pongs_received: u64,
// }
// pub async fn handle_ping(address: &str) -> Result<PingResponse, std::io::Error> {
//     let response = reqwest::Client::new()
//         .get(&format!(
//             "{}/burst?requestType=getBlockchainStatus",
//             address
//         ))
//         .send()
//         .await
//         .expect("Failed to execute request.");
//     let mut pingresponse = PingResponse {
//         number_pings_sent: 1,
//         number_pongs_received: 0,
//     };
//     if response.status().is_success() {
//         pingresponse.number_pongs_received += 1;
//     }
//     dbg!(response);
//     Ok(pingresponse)
// }
