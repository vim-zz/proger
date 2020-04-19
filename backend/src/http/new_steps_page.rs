use log::debug;

use actix_web::{
    web::Json,
    HttpResponse,
    Error,
};
use proger_core::protocol::{request, response};

pub async fn new_steps_page(payload: Json<request::NewStepsPage>) -> Result<HttpResponse, Error> {
    debug!("new steps page request: {:?}", payload);
    Ok(HttpResponse::Ok().json(response::PageAccess{
        admin_secret: "ADMIN_SECRET".to_string(), 
        private_link: "PRIVATE_LINK".to_string(),
    }))
}