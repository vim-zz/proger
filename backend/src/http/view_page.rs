use log::debug;

use actix_web::{
    web::{
        Path
    },
    HttpResponse,
    Error,
};

pub async fn view_page(id: Path<String>) -> Result<HttpResponse, Error> {
    debug!("here it is: {:?}", id);
    Ok(HttpResponse::Ok().finish())
}