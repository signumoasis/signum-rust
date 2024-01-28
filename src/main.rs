use clap::{Parser, Subcommand};
use signum_cli::sub_commands::server_info::handle_serverinfo_getmyinfo;

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
        Some(Commands::Ping) => {
            println!("Pinging server?")
        }
        Some(Commands::Serverinfo) => {
            println!("Get server info");
            let r = handle_serverinfo_getmyinfo(&cli.server_address).await;
            match r {
                Ok(r) => println!("{:#?}",r),
                Err(e) => println!("Unable to get server info:\n\t{}", e),
            }
        }
        Some(Commands::Peers) => {
            println!("Get peers");
        }
        None => {}
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    server_address: String,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ping,
    Serverinfo,
    Peers,
}
