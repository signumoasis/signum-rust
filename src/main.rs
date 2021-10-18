use clap::{App, Arg, SubCommand};
fn main() {
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
        .subcommand(
            SubCommand::with_name("serverinfo")
                .about("Gets information about a server")
                .version("0.1.0")
                .subcommand(
                    SubCommand::with_name("getmyinfo")
                        .about("Displays information about this server.")
                        .version("0.1.0"),
                )
                .subcommand(
                    SubCommand::with_name("getpeers")
                        .about("Lists this server's peers.")
                        .version("0.1.0"),
                ),
        )
        .subcommand(
            SubCommand::with_name("serverpeerinfo")
                .about("Gets information about a server's peers")
                .version("0.1.0")
                .subcommand(
                    SubCommand::with_name("getmypeerinfo")
                        .about("Displays information about this server.")
                        .version("0.1.0"),
                ),
        )
        .get_matches();

    let config = matches.value_of("config").unwrap_or("signumcli.conf");
    println!("Value for config: {}", config);

    println!(
        "Using node URL: {}",
        matches.value_of("URL").unwrap_or("localhost:8125")
    );

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("All of the info. All of it."),
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
            ("getmyinfo", Some(_sub_m)) => println!("getting the server info"),
            ("getpeers", Some(_sub_m)) => println!("here's all the peers"),
            _ => {}
        },
        ("serverpeerinfo", Some(sub_m)) => {
            if let ("getmypeerinfo", Some(_sub_m)) = sub_m.subcommand() {
                println!("getting the server's peer info")
            }
        }
        _ => {}
    }
}
