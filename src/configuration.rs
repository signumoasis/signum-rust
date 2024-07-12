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

#[allow(dead_code)]
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
    //pub historical_moments: HistoricalMoments,
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

/// This settings struct represents any overrides for the historical moments. All values are optional.
#[derive(Clone, Debug, Deserialize)]
pub struct HistoricalMoments {
    #[serde(default = "HistoricalMoments::genesis")]
    pub genesis: u32,
    #[serde(default = "HistoricalMoments::reward_recipient_enable")]
    pub reward_recipient_enable: u32,
    #[serde(default = "HistoricalMoments::digital_goods_store_enable")]
    pub digital_goods_store_enable: u32,
    #[serde(default = "HistoricalMoments::automated_transaction_enable")]
    pub automated_transaction_enable: u32,
    #[serde(default = "HistoricalMoments::automated_transaction_fix_1")]
    pub automated_transaction_fix_1: u32,
    #[serde(default = "HistoricalMoments::automated_transaction_fix_2")]
    pub automated_transaction_fix_2: u32,
    #[serde(default = "HistoricalMoments::automated_transaction_fix_3")]
    pub automated_transaction_fix_3: u32,
    #[serde(default = "HistoricalMoments::pre_poc2")]
    pub pre_poc2: u32,
    #[serde(default = "HistoricalMoments::poc2_enable")]
    pub poc2_enable: u32,
    #[serde(default = "HistoricalMoments::sodium_enable")]
    pub sodium_enable: u32,
    #[serde(default = "HistoricalMoments::signum_name_change")]
    pub signum_name_change: u32,
    #[serde(default = "HistoricalMoments::poc_plus_enable")]
    pub poc_plus_enable: u32,
    #[serde(default = "HistoricalMoments::speedway_enable")]
    pub speedway_enable: u32,
    #[serde(default = "HistoricalMoments::smart_token_enable")]
    pub smart_token_enable: u32,
    #[serde(default = "HistoricalMoments::smart_fees_enable")]
    pub smart_fees_enable: u32,
    #[serde(default = "HistoricalMoments::smart_ats_enable")]
    pub smart_ats_enable: u32,
    #[serde(default = "HistoricalMoments::automated_transaction_fix_4")]
    pub automated_transaction_fix_4: u32,
    #[serde(default = "HistoricalMoments::distribution_fix_enable")]
    pub distribution_fix_enable: u32,
    #[serde(default = "HistoricalMoments::pk_freeze")]
    pub pk_freeze: u32,
    #[serde(default = "HistoricalMoments::pk_freeze_2")]
    pub pk_freeze_2: u32,
    #[serde(default = "HistoricalMoments::smart_alias_enable")]
    pub smart_alias_enable: u32,
    #[serde(default = "HistoricalMoments::next_fork")]
    pub next_fork: u32,
    //pub reward_recipient_enable: Option<u32>,
    //pub digital_goods_store_enable: Option<u32>,
    //pub automated_transaction_enable: Option<u32>,
    //pub automated_transaction_fix_1: Option<u32>,
    //pub automated_transaction_fix_2: Option<u32>,
    //pub automated_transaction_fix_3: Option<u32>,
    //pub pre_poc2: Option<u32>,
    //pub poc2_enable: Option<u32>,
    //pub sodium_enable: Option<u32>,
    //pub signum_name_change: Option<u32>,
    //pub poc_plus_enable: Option<u32>,
    //pub speedway_enable: Option<u32>,
    //pub smart_token_enable: Option<u32>,
    //pub smart_fees_enable: Option<u32>,
    //pub smart_ats_enable: Option<u32>,
    //pub automated_transaction_fix_4: Option<u32>,
    //pub distribution_fix_enable: Option<u32>,
    //pub pk_freeze: Option<u32>,
    //pub pk_freeze_2: Option<u32>,
    //pub smart_alias_enable: Option<u32>,
    //pub next_fork: Option<u32>,
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
    #[serde(default = "PeerToPeerSettings::default_value_bootstrap_peers")]
    pub bootstrap_peers: Vec<PeerAddress>,
    /// Address that peers should attempt to connect to.
    #[serde(default = "PeerToPeerSettings::default_value_my_address")]
    pub my_address: String,
    /// A string indicating the platform in use. Often set to a signum address for SNR rewards.
    #[serde(default = "PeerToPeerSettings::default_value_platform")]
    pub platform: String,
    /// Whether or not peers should pass along your address to their own peers.
    #[serde(default = "PeerToPeerSettings::default_value_share_address")]
    pub share_address: bool,
    /// The name of the network to which this node is connecting.
    #[serde(default = "PeerToPeerSettings::default_value_network_name")]
    pub network_name: String,
    /// The address to which SNR awards should be paid. Currently unused on the network.
    #[serde(default = "PeerToPeerSettings::default_value_snr_reward_address")]
    pub snr_reward_address: String,
}

// Defaults for PeerToPeerSettings
impl PeerToPeerSettings {
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
}

// Defaults for HistoricalMoments
impl HistoricalMoments {
    fn genesis() -> u32 {
        0
    }
    fn reward_recipient_enable() -> u32 {
        6_500
    }
    fn digital_goods_store_enable() -> u32 {
        11_800
    }
    fn automated_transaction_enable() -> u32 {
        49_200
    }
    fn automated_transaction_fix_1() -> u32 {
        67_000
    }
    fn automated_transaction_fix_2() -> u32 {
        92_000
    }
    fn automated_transaction_fix_3() -> u32 {
        255_000
    }
    fn pre_poc2() -> u32 {
        500_000
    }
    fn poc2_enable() -> u32 {
        502_000
    }
    fn sodium_enable() -> u32 {
        765_000
    }
    fn signum_name_change() -> u32 {
        875_500
    }
    fn poc_plus_enable() -> u32 {
        878_000
    }
    fn speedway_enable() -> u32 {
        941_100
    }
    fn smart_token_enable() -> u32 {
        1_029_000
    }
    fn smart_fees_enable() -> u32 {
        1_029_000
    }
    fn smart_ats_enable() -> u32 {
        1_029_000
    }
    fn automated_transaction_fix_4() -> u32 {
        1_051_900
    }
    fn distribution_fix_enable() -> u32 {
        1_051_900
    }
    fn pk_freeze() -> u32 {
        1_099_400
    }
    fn pk_freeze_2() -> u32 {
        1_150_000
    }
    fn smart_alias_enable() -> u32 {
        1_150_000
    }
    fn next_fork() -> u32 {
        u32::MAX
    }
}
