use actix_web::{web::Json, HttpResponse};

use super::request_models;

#[tracing::instrument(skip_all)]
pub async fn signum_api_handler(request_object: Json<request_models::RequestType>) -> HttpResponse {
    tracing::debug!("Request Object: {:#?}", &request_object);
    HttpResponse::Ok().body("howdy")
}
