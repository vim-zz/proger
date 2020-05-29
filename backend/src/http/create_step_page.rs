use crate::StorageExecutor;
use actix_web::{
    web::{Data, Json},
    Error, HttpResponse,
};
use proger_core::protocol::{request::CreateStepPage, response::PageAccess};

use crate::storage::storage_driver::{StorageCmd, StorageDriver};
use actix::Addr;

pub async fn create_step_page<T: StorageDriver>(
    payload: Json<CreateStepPage>,
    storage: Data<Addr<StorageExecutor<T>>>,
) -> Result<HttpResponse, Error> {
    let result = storage
        .into_inner()
        .send(StorageCmd::CreateStepPage(payload.into_inner()))
        .await?;

    match result {
        Ok(page) => Ok(HttpResponse::Ok().json(PageAccess {
            admin_secret: page.secret,
            link: page.link,
        })),
        Err(e) => Ok(HttpResponse::BadRequest().finish()),
    }
}
