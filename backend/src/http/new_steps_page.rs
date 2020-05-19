use crate::StorageExecutor;
use actix_web::{
    web::{Data, Json},
    Error, HttpResponse,
};
use log::debug;
use proger_core::protocol::{request::NewStepsPage, response::PageAccess};

use crate::storage::storage_driver::{StorageCmd, StorageDriver};
use actix::Addr;

pub async fn new_steps_page<T: StorageDriver>(
    payload: Json<NewStepsPage>,
    storage: Data<Addr<StorageExecutor<T>>>,
) -> Result<HttpResponse, Error> {
    debug!("new steps page request: {:?}", payload);
    let _result = storage
        .into_inner()
        .send(StorageCmd::CreateStepsPage(payload.into_inner()))
        .await?;
    Ok(HttpResponse::Ok().json(PageAccess {
        admin_secret: "ADMIN_SECRET".to_string(),
        link: "PRIVATE_LINK".to_string(),
    }))
}
