use clap::{Parser, Subcommand, ValueEnum};
use signum_rs::old_api::network::PeerStates as SignumPeerStates;
use signum_rs::old_api::network::{get_my_info, get_my_peer_info, get_peers};
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(Commands::Peers { active, state }) => {
            // [`signum_rs::api::network::PeerStates`] does not implement [`clap::ValueEnum`]
            // so we need to mask it out with a matching local enum that does.
            let state: SignumPeerStates = match state {
                PeerStates::All => SignumPeerStates::All,
                PeerStates::Connected => SignumPeerStates::Connected,
                PeerStates::Disconnected => SignumPeerStates::Disconnected,
                PeerStates::NonConnected => SignumPeerStates::NonConnected,
            };
            let result = get_peers(&cli.server_address, active, &state).await;
            println!("Get peers: {:#?}", result);
        }
        Some(Commands::Ping) => {
            println!("Pinging server?")
        }
        Some(Commands::Serverinfo) => {
            let result = get_my_info(&cli.server_address).await;
            match result {
                Ok(result) => println!("{:#?}", result),
                Err(e) => println!("Unable to get server info:\n\t{}", e),
            }
        }
        Some(Commands::Serverpeerinfo) => {
            let result = get_my_peer_info(&cli.server_address).await;
            match result {
                Ok(result) => println!("{:#?}", result),
                Err(e) => println!("Unable to get server peer info:\n\t{}", e),
            }
        }
        None => {}
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    server_address: String,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Peers {
        #[arg(short, long, default_value_t = false)]
        active: bool,
        #[arg(value_enum, default_value_t = PeerStates::All)]
        state: PeerStates,
    },
    Ping,
    Serverinfo,
    Serverpeerinfo,
}

#[derive(Clone, Debug, ValueEnum)]
enum PeerStates {
    All,
    Connected,
    Disconnected,
    NonConnected,
}
