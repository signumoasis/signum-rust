use actix_web::HttpResponse;

#[tracing::instrument(skip_all)]
pub async fn signum_api_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}
