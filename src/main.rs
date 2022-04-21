use clap::{App, Arg};
use signum_cli::sub_commands::{
    ping::{self, handle_ping, ClapAppPingExtension},
    server_info::{get_my_info::handle_serverinfo_getmyinfo, ClapAppServerInfoExtensions},
};

#[tokio::main]
async fn main() {
    let matches = App::new("Signum CLI")
        .version("0.1.0")
        .author("damccull")
        .about("Provides a cli-based status of your node.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("URL")
                .help("The Signum node's API url (e.g. https://canada.signum.network:8125)")
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .add_server_info_subcommands()
        .add_ping_subcommands()
        .get_matches();

    let config = matches.value_of("config").unwrap_or("signumcli.conf");
    println!("Value for config: {}", config);

    let address = matches.value_of("URL").unwrap_or("http://localhost:8125");

    println!("Using node URL: {}", address);

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("All of the info. All of it."),
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    match matches.subcommand() {
        ("serverinfo", Some(sub_m)) => match sub_m.subcommand() {
            ("getmyinfo", Some(_sub_m)) => {
                println!("getting the server info...");
                if let Ok(result) = handle_serverinfo_getmyinfo(address).await {
                    println!(
                        "Address: {}, Host: {}, UUID: {}, Request Processing Time: {}",
                        result.address, result.host, result.uuid, result.request_processing_time
                    );
                } else {
                    println!("Unable to connect for some reason.");
                }
            }
            ("getpeers", Some(_sub_m)) => println!("here's all the peers"),
            _ => {}
        },
        ("serverpeerinfo", Some(sub_m)) => {
            if let ("getmypeerinfo", Some(_sub_m)) = sub_m.subcommand() {
                println!("getting the server's peer info")
            }
        }
        (ping::SUBCOMMAND_NAME, Some(sub_m)) => {
            if let Ok(result) = handle_ping(address).await {
                println!("Pinged the server: {:#?}", result);
            } else {
                println!("Unable to connect to server...")
            }
            // println!("Ping called");
            // dbg!(sub_m);
        }
        _ => {}
    }
}
