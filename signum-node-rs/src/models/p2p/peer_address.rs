use std::{fmt::Display, str::FromStr};

use anyhow::Context;
use reqwest::Url;
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(
    Clone, Debug, Default, DeserializeFromStr, Eq, Hash, PartialEq, SerializeDisplay, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct PeerAddress(pub(crate) String);

impl PeerAddress {
    pub fn to_url(&self) -> Url {
        Url::parse(format!("http://{}", &self.0).as_str())
            .context("Couldn't parse url")
            .unwrap()
    }
}
impl Display for PeerAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FromStr for PeerAddress {
    type Err = anyhow::Error;

    #[tracing::instrument(name = "Parsing Peer Address")]
    fn from_str(value: &str) -> Result<Self, Self::Err> {
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
        tracing::trace!("Parsed: {}", &address);
        Ok(PeerAddress(address))
    }
}

#[cfg(test)]
mod test {
    use crate::models::p2p::PeerAddress;

    #[test]
    fn peer_address_fromstr_succeeds_for_valid_urls() {
        // Prepare
        let example_443 = PeerAddress("p2p.signumoasis.xyz:443".to_string());
        let example_80 = PeerAddress("p2p.signumoasis.xyz:80".to_string());
        let example_8123 = PeerAddress("p2p.signumoasis.xyz:8123".to_string());
        let example_ipv4 = PeerAddress("127.0.0.1:8123".to_string());
        let example_ipv6 = PeerAddress("[::1]:8123".to_string());
        let urls = vec![
            ("https://p2p.signumoasis.xyz".to_string(), &example_8123),
            ("http://p2p.signumoasis.xyz".to_string(), &example_8123),
            ("https://p2p.signumoasis.xyz:443".to_string(), &example_443),
            ("http://p2p.signumoasis.xyz:80".to_string(), &example_80),
            ("p2p.signumoasis.xyz".to_string(), &example_8123),
            ("p2p.signumoasis.xyz:80".to_string(), &example_80),
            ("127.0.0.1".to_string(), &example_ipv4),
            ("127.0.0.1:8123".to_string(), &example_ipv4),
            ("[::1]".to_string(), &example_ipv6),
            ("[::1]:8123".to_string(), &example_ipv6),
        ];

        // Act / Assert
        for (u, e) in urls.into_iter() {
            assert_eq!(u.parse::<PeerAddress>().unwrap(), *e, "Failed on `{}`", u);
        }
    }

    #[test]
    fn peer_address_fromstr_fails_for_invalid_urls() {
        // Prepare
        let urls = vec!["[:::1]".to_string(), "[:::1]:8123".to_string()];

        // Act / Assert
        for u in urls.into_iter() {
            u.parse::<PeerAddress>().unwrap_err();
        }
    }
}
