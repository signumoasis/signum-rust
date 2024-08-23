use std::{str::FromStr, time::Duration};

use anyhow::{Context, Result};
use num_bigint::BigUint;
use reqwest::Response;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::models::{
    p2p::{B1Block, PeerAddress, PeerInfo},
    Block,
};

use super::{BasicPeerClient, DownloadResult, PeerCommunicationError};

// TODO: Refactor this to use GRPC and actually handle other oasis peers. Right now it's just a B1Peer clone
#[derive(Debug, Default)]
pub struct OasisPeer {
    peer: PeerAddress,
    pub announced_address: Option<PeerAddress>,
    pub application: String,
    pub version: String,
    pub platform: Option<String>,
    pub share_address: bool,
    pub network_name: String,
    pub oasis_info: OasisPeerInfo,
}

impl OasisPeer {
    pub fn new(peer: PeerAddress) -> Self {
        Self {
            peer,
            ..Default::default()
        }
    }

    pub async fn post_peer_request(
        &self,
        request_body: &Value,
        timeout: Option<Duration>,
    ) -> Result<Response, reqwest::Error> {
        let mut client = reqwest::Client::new().post(self.peer.to_url());
        if let Some(timeout) = timeout {
            client = client.timeout(timeout);
        }
        client = client.header("User-Agent", "BRS/3.8.2").json(&request_body);

        client.send().await
    }
}

impl BasicPeerClient for OasisPeer {
    fn address(&self) -> PeerAddress {
        self.peer.clone()
    }

    async fn get_blocks_from_height(
        &self,
        height: u64,
        number_of_blocks: u32,
    ) -> Result<DownloadResult, PeerCommunicationError> {
        let thebody = json!({
            "protocol": "B1",
            "requestType": "getBlocksFromHeight",
            "height": height,
            "numBlocks": number_of_blocks,
        });

        tracing::trace!(
            "Downloading blocks {} through {} from {}.",
            height,
            number_of_blocks,
            &self.address()
        );

        let response = self.post_peer_request(&thebody, None).await;

        let response = match response {
            Ok(result) => Ok(result),
            Err(e) if e.is_connect() => Err(PeerCommunicationError::ConnectionError(e)),
            Err(e) if e.is_timeout() => Err(PeerCommunicationError::ConnectionTimeout(e)),
            Err(e) => Err(PeerCommunicationError::UnexpectedError(
                Err(e).context("could not get a response")?,
            )),
        }?;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct NextBlocks {
            next_blocks: Vec<B1Block>,
        }
        tracing::debug!(
            "Blocks Downloaded for {}:\n{:#?}",
            &self.peer,
            response.json::<NextBlocks>().await
        );

        let result = DownloadResult {
            peer: self.peer.clone(),
            start_height: height,
            number_of_blocks,
            blocks: Vec::<Block>::new(),
        };

        //TODO: Process the blocks just downloaded and return the correct Result
        // - OK if all in this subchain are good
        // - Connection error for any connectivity issues
        // - Parse or Verification error for bad blocks
        Ok(result)
    }

    async fn get_peers(&self) -> Result<Vec<PeerAddress>, anyhow::Error> {
        let thebody = json!({
            "protocol": "B1",
            "requestType": "getPeers",
        });

        let response = self.post_peer_request(&thebody, None).await?;

        tracing::trace!("Parsing peers...");
        #[derive(Debug, serde::Deserialize)]
        struct PeerContainer {
            #[serde(rename = "peers")]
            peers: Vec<PeerAddress>,
        }
        let result = response.json::<PeerContainer>().await?;
        tracing::trace!("Peers successfully parsed: {:#?}", &result);
        Ok(result.peers)
    }

    /// Get the cumulative difficulty from the peer.
    async fn get_peer_cumulative_difficulty(&self) -> Result<BigUint> {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct CumulativeDifficultyResponse {
            pub cumulative_difficulty: String,
            // #[serde(rename = "blockchainheight")]
            // pub _blockchain_height: u64,
        }

        let thebody = json!({
            "protocol": "B1",
            "requestType": "getCumulativeDifficulty",
        });

        let response = self
            .post_peer_request(&thebody, Some(Duration::from_secs(2)))
            .await;

        let response = match response {
            Ok(r) => Ok(r),
            Err(e) if e.is_connect() => Err(PeerCommunicationError::ConnectionError(e)),
            Err(e) if e.is_timeout() => Err(PeerCommunicationError::ConnectionTimeout(e)),
            Err(e) => Err(PeerCommunicationError::UnexpectedError(
                Err(e).context("could not get a response")?,
            )),
        }?;

        let values = match response.json::<CumulativeDifficultyResponse>().await {
            Ok(i) => Ok(i),
            Err(e) => Err(anyhow::anyhow!(
                "Error getting cumulative difficulty: {:#?}",
                e
            )),
        }?;

        let out = BigUint::from_str(&values.cumulative_difficulty)
            .context("couldn't convert string to a BigUint")?;

        Ok(out)
    }

    async fn get_peer_info(&self) -> Result<(PeerInfo, String), PeerCommunicationError> {
        todo!()
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct OasisPeerInfo {
    some_thing: String,
}
