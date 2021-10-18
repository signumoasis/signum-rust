use clap::{App, Arg};
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
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let config = matches.value_of("config").unwrap_or("signumcli.conf");
    println!("Value for config: {}", config);

    println!("Using node URL: {}", matches.value_of("URL").unwrap());

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("All of the info. All of it."),
    }
}
