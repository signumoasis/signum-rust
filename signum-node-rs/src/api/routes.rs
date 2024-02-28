use actix_web::{http::header::ContentType, HttpResponse};

#[tracing::instrument(skip_all)]
pub async fn home() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("home.html"))
}
