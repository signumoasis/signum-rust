use clap::SubCommand;

pub mod get_my_peer_info;

pub trait ClapAppServerPeerInfoExtensions {
    fn add_server_info_subcommands(self) -> Self;
}
impl<'a, 'b> ClapAppServerPeerInfoExtensions for clap::App<'a, 'b> {
    fn add_server_info_subcommands(self) -> Self {
        // Build this command's subcommands
        let mut scmd = SubCommand::with_name("serverinfo")
            .about("Gets information about a server")
            .version("0.1.0");
        //scmd = subcommand_get_my_peer_info(scmd);

        // Attach the subcommands to this command and send it back to the called
        self.subcommand(scmd)

        // self.subcommand(
        //     SubCommand::with_name("serverpeerinfo")
        //         .about("Gets information about a server's peers")
        //         .version("0.1.0")
        //         .subcommand(
        //             SubCommand::with_name("getmypeerinfo")
        //                 .about("Displays information about this server.")
        //                 .version("0.1.0"),
        //         ),
        // )
    }
}
