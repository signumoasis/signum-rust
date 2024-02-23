use crate::configuration::Settings;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        todo!()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn run_until_stopped(self) -> hyper::Result<()> {
        self.server.await
    }
}
