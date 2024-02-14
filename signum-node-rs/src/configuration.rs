use serde::Deserialize;
use sqlx::sqlite::SqliteConnectOptions;

use crate::models::p2p::PeerAddress;

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Get the base execution director
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    // Set the configuration file
    let configuration_file = "configuration.yml";

    let settings = config::Config::builder()
        // .add_defaults()?
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

// impl<St: config::builder::BuilderState> ConfigBuilderExtensions for ConfigBuilder<St> {
//     fn add_defaults(self) -> Result<Self, config::ConfigError> {
//         self.set_default(
//             "settings.p2p.bootstrap_peers",
//             vec![PeerAddress::from_str("us-east.signum.network:8123")?],
//         )
//     }
// }

/// Settings for the node.
#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub node: NodeSettings,
    pub p2p: PeerToPeerSettings,
}

/// Database settings.
#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseSettings {
    pub filename: String,
}

impl DatabaseSettings {
    pub fn get_writable_db(&self) -> SqliteConnectOptions {
        SqliteConnectOptions::new()
            .filename(&self.filename)
            .optimize_on_close(true, None)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .create_if_missing(true)
    }
    pub fn get_read_only_db(&self) -> SqliteConnectOptions {
        SqliteConnectOptions::new()
            .filename(&self.filename)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .read_only(true)
            .create_if_missing(true)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct NodeSettings {
    pub my_address: String,
    pub cash_back_id: String,
    pub network: String,
}

/// Peer to Peer settings.
#[derive(Clone, Debug, Deserialize)]
pub struct PeerToPeerSettings {
    /// Peer addresses to use if none are in the database already.
    pub bootstrap_peers: Vec<PeerAddress>,
}
