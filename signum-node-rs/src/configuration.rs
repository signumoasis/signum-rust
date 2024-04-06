use std::str::FromStr;

use serde::Deserialize;
use surrealdb::{
    engine::any::{self, Any},
    opt::auth::Root,
    Surreal,
};

use crate::models::{datastore::Datastore, p2p::PeerAddress};

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Get the base execution director
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    // Set the configuration file
    let configuration_file = "configuration.yml";

    let settings = config::Config::builder()
        //add values from a file
        .add_source(config::File::from(base_path.join(configuration_file)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    let settings: Result<Settings, config::ConfigError> = settings.try_deserialize();
    tracing::debug!("Settings values: {:#?}", &settings);
    settings
}

trait ConfigBuilderExtensions {
    fn add_defaults(self) -> Result<Self, config::ConfigError>
    where
        Self: Sized;
}

/// Settings for the node.
#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub srs_api: SrsApiSettings,
    pub database: DatabaseSettings,
    pub node: NodeSettings,
    pub p2p: PeerToPeerSettings,
}

/// Settings for the signum-style API.
#[derive(Clone, Debug, Deserialize)]
pub struct SrsApiSettings {
    pub base_url: String,
    pub listen_address: String,
    pub listen_port: u16,
}

/// Database settings.
#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseSettings {
    pub filename: String,
}

impl DatabaseSettings {
    #[tracing::instrument(skip_all)]
    pub async fn get_db(&self) -> Result<Datastore, anyhow::Error> {
        let db = any::connect(&self.filename).await?;
        // let db = any::connect(format!("speedb:{}", self.filename)).await?;

        if !&self.filename.starts_with("speedb:")
            && !self.filename.starts_with("file:")
            && !&self.filename.starts_with("mem:")
        {
            db.signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        }

        let namespace = "signum";
        let database = "signum";
        db.use_ns(namespace).use_db(database).await?;

        tracing::info!(
            "Opened surrealdb file db {}, using namespace {} and database {}",
            &self.filename,
            namespace,
            database
        );

        tracing::info!("Initializing database");
        let db = initialize_database(db).await?;

        Ok(Datastore::new(db))
    }
}

#[tracing::instrument(skip_all)]
async fn initialize_database(db: Surreal<Any>) -> Result<Surreal<Any>, anyhow::Error> {
    tracing::info!("Defining unique index on announced_address field");
    db.query(
        r#"
            DEFINE INDEX unique_announced_address ON peer COLUMNS announced_address UNIQUE
        "#,
    )
    .await?;

    Ok(db)
}

#[derive(Clone, Debug, Deserialize)]
pub struct NodeSettings {
    pub cash_back_id: String,
    pub network: String,
}

/// Peer to Peer settings.
#[derive(Clone, Debug, Deserialize)]
pub struct PeerToPeerSettings {
    /// Peer addresses to use if none are in the database already.
    #[serde(default = "default_value_bootstrap_peers")]
    pub bootstrap_peers: Vec<PeerAddress>,
    /// Address that peers should attempt to connect to.
    #[serde(default = "default_value_my_address")]
    pub my_address: String,
    /// A string indicating the platform in use. Often set to a signum address for SNR rewards.
    #[serde(default = "default_value_platform")]
    pub platform: String,
    /// Whether or not peers should pass along your address to their own peers.
    #[serde(default = "default_value_share_address")]
    pub share_address: bool,
    /// The name of the network to which this node is connecting.
    #[serde(default = "default_value_network_name")]
    pub network_name: String,
    /// The address to which SNR awards should be paid. Currently unused on the network.
    #[serde(default = "default_value_snr_reward_address")]
    pub snr_reward_address: String,
}

fn default_value_bootstrap_peers() -> Vec<PeerAddress> {
    vec![
        PeerAddress::from_str("australia.signum.network:8123")
            .expect("could not parse bootstrap ip address `australia.signum.network:8123`"),
        PeerAddress::from_str("brazil.signum.network:8123")
            .expect("could not parse bootstrap ip address `brazil.signum.network:8123`"),
        PeerAddress::from_str("canada.signum.network:8123")
            .expect("could not parse bootstrap ip address `canada.signum.network:8123`"),
        PeerAddress::from_str("europe.signum.network:8123")
            .expect("could not parse bootstrap ip address `europe.signum.network:8123`"),
        PeerAddress::from_str("europe1.signum.network:8123")
            .expect("could not parse bootstrap ip address `europe1.signum.network:8123`"),
        PeerAddress::from_str("europe2.signum.network:8123")
            .expect("could not parse bootstrap ip address `europe2.signum.network:8123`"),
        PeerAddress::from_str("europe3.signum.network:8123")
            .expect("could not parse bootstrap ip address `europe3.signum.network:8123`"),
        PeerAddress::from_str("latam.signum.network:8123")
            .expect("could not parse bootstrap ip address `latam.signum.network:8123`"),
        PeerAddress::from_str("singapore.signum.network:8123")
            .expect("could not parse bootstrap ip address `singapore.signum.network:8123`"),
        PeerAddress::from_str("ru.signum.network:8123")
            .expect("could not parse bootstrap ip address `ru.signum.network:8123`"),
        PeerAddress::from_str("us-central.signum.network:8123")
            .expect("could not parse bootstrap ip address `us-central.signum.network:8123`"),
        PeerAddress::from_str("us-east.signum.network:8123")
            .expect("could not parse bootstrap ip address `us-east.signum.network:8123`"),
    ]
}

fn default_value_my_address() -> String {
    //TODO: Figure out a way to get external IP and populate it
    String::new()
}

fn default_value_platform() -> String {
    String::new()
}

fn default_value_share_address() -> bool {
    true
}

fn default_value_network_name() -> String {
    "Signum".to_string()
}

fn default_value_snr_reward_address() -> String {
    String::new()
}
