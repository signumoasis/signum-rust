use configuration::DatabaseSettings;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub mod api;
pub mod configuration;
pub mod models;
pub mod peers;
pub mod telemetry;
pub mod workers;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

/// Get a read-only database connection pool.
pub fn get_read_only_db_pool(configuration: &DatabaseSettings) -> SqlitePool {
    SqlitePoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.get_read_only_db())
}
/// Get a writable database connection pool.
pub fn get_writable_db_pool(configuration: &DatabaseSettings) -> SqlitePool {
    SqlitePoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .max_connections(1)
        .connect_lazy_with(configuration.get_writable_db())
}
