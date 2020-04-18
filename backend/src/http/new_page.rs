use log::debug;

use actix_web::{
    web::Json,
    HttpResponse,
    Error,
};
use proger_core::protocol::{request, response};

pub async fn new_page(payload: Json<request::NewPage>) -> Result<HttpResponse, Error> {
    debug!("new page request: {:?}", payload);
    Ok(HttpResponse::Ok().json(response::PageID("banana_id".to_string())))
}