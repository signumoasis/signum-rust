use actix_web::HttpResponse;

pub mod configuration;
pub mod models;
pub mod peers;
pub mod srs_api;
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

#[tracing::instrument(skip_all)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
