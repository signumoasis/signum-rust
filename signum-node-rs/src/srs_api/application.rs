use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Context;
use surrealdb::{engine::any::Any, Surreal};
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::{PeerToPeerSettings, Settings},
    health_check,
    srs_api::signum_api_handler,
};

pub struct SrsApiApplication {
    port: u16,
    server: Server,
}

impl SrsApiApplication {
    pub async fn build(
        configuration: Settings,
        database: Surreal<Any>,
    ) -> Result<Self, anyhow::Error> {
        // let connection_pool = get_connection_pool(&configuration.database)?;
        // let database = configuration
        //     .database
        //     .get_db()
        //     .await
        //     .context("could not get database connection")?;

        let address = format!(
            "{}:{}",
            configuration.srs_api.listen_address, configuration.srs_api.listen_port
        );

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            database,
            configuration.srs_api.base_url,
            configuration.p2p.clone(),
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

// fn get_connection_pool(configuration: &DatabaseSettings) -> Result<SqlitePool, anyhow::Error> {
//     Ok(SqlitePoolOptions::new().connect_lazy_with(configuration.get_writable_db()?))
// }

pub struct ApplicationBaseUrl(pub String);

async fn run(
    listener: TcpListener,
    db: Surreal<Any>,
    base_url: String,
    p2p_settings: PeerToPeerSettings,
) -> Result<Server, anyhow::Error> {
    let db = Data::new(db);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let p2p_settings = Data::new(p2p_settings);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/{allroutes:.*}", web::post().to(signum_api_handler))
            .app_data(db.clone())
            .app_data(base_url.clone())
            .app_data(p2p_settings.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
