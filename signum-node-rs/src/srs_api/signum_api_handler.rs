use actix_web::{error::ErrorInternalServerError, web::Json, HttpResponse, Responder};

use crate::srs_api::{add_peers, get_peers};

use super::request_models;

#[tracing::instrument(skip_all)]
pub async fn signum_api_handler(
    request_object: Json<request_models::RequestType>,
) -> impl Responder {
    tracing::debug!("Request Object: {:#?}", &request_object);
    let _response = match request_object.0 {
        request_models::RequestType::AddPeers { peers } => add_peers::add_peers_handler(peers),
        request_models::RequestType::GetPeers {} => get_peers::get_peers_handler(),
    }
    .map_err(ErrorInternalServerError);
    HttpResponse::Ok().finish()
}