pub mod p2p {
    use std::str::FromStr;

    use reqwest::Url;
    use serde_with::DeserializeFromStr;

    #[derive(Debug)]
    pub struct PeerInfo;

    #[derive(Clone, Debug, DeserializeFromStr, Eq, Hash, PartialEq)]
    pub struct PeerAddress(String);
    impl FromStr for PeerAddress {
        type Err = anyhow::Error;

        #[tracing::instrument(name = "models::P2P::PeerAddress.try_from()")]
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            //Verify proper URL parse
            //Extract host and port
            //Return as `host:port`

            // Remove any existing scheme by splitting on "://" if it exists and only taking the right half
            // or taking the base value if no "://" exists
            let value = value.split_once("://").unwrap_or(("", value)).1;

            // Parse value with dummy scheme to validate proper url format.
            // Don't use a real scheme here because `[Url::parse]` will output
            // that scheme's default port as "None" for the address.
            let url = Url::parse(&format!("dummyscheme://{}", value))?;

            let host = url
                .host()
                .ok_or_else(|| anyhow::anyhow!("invalid url: {}", value))?;
            let port = url.port().unwrap_or(8123);

            let address = format!("{}:{}", host, port);
            Ok(PeerAddress(address))
        }
    }

    #[derive(Debug)]
    pub struct BlockId;

    #[derive(Debug)]
    pub struct Transaction;
}
