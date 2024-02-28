use actix_web::HttpResponse;

#[tracing::instrument(skip_all)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
