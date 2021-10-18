use clap::SubCommand;

use self::{get_my_info::add_subcommand_server_info, get_peers::subcommand_get_peers};

pub mod get_my_info;
pub mod get_peers;

pub trait ClapAppServerInfoExtensions {
    fn add_server_info_subcommands(self) -> Self;
}
impl<'a, 'b> ClapAppServerInfoExtensions for clap::App<'a, 'b> {
    fn add_server_info_subcommands(self) -> Self {
        // Build this command's subcommands
        let mut scmd = SubCommand::with_name("serverinfo")
            .about("Gets information about a server")
            .version("0.1.0");
        scmd = add_subcommand_server_info(scmd);
        scmd = subcommand_get_peers(scmd);

        // Attach the subcommands to this command and send it back to the called
        self.subcommand(scmd)
    }
}
