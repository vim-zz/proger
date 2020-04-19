use log::debug;

use actix_web::{
    web::{
        Json,
        Path
    },
    HttpResponse,
    Error,
};
use proger_core::protocol::{request, response};

pub async fn set_steps_page(id: Path<String>, payload: Json<request::SetStepsPage>) -> Result<HttpResponse, Error> {
    debug!("set steps page {} request: {:?}", id, payload);
    Ok(HttpResponse::Ok().finish())
}