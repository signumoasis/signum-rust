use config::ConfigBuilder;
use serde::Deserialize;

use crate::models::p2p::PeerAddress;

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Get the base execution director
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    // Set the configuration file
    let configuration_file = "configuration.yml";

    let settings = config::Config::builder()
        .add_defaults()?
        //add values from a file
        .add_source(config::File::from(base_path.join(configuration_file)))
        .build()?;

    let settings = settings.try_deserialize::<Settings>();
    tracing::debug!("Settings values: {:#?}", &settings);
    settings
}

trait ConfigBuilderExtensions {
    fn add_defaults(self) -> Result<Self, config::ConfigError>
    where
        Self: Sized;
}

impl<St: config::builder::BuilderState> ConfigBuilderExtensions for ConfigBuilder<St> {
    fn add_defaults(self) -> Result<Self, config::ConfigError> {
        self.set_default(
            "settings.p2p.bootstrap_peers",
            "us-east.signum.network:8123".to_string(),
        )
    }
}

/// Settings for the node.
#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub p2p: PeerToPeerSettings,
}

/// Peer to Peer settings.
#[derive(Clone, Debug, Deserialize)]
pub struct PeerToPeerSettings {
    /// Peer addresses to use if none are in the database already.
    pub bootstrap_peers: Vec<PeerAddress>,
}
