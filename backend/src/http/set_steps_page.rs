use log::debug;
use actix_web::{
    web::{Json, Data},
    HttpResponse,
    Error,
};
use proger_core::protocol::{
    request::SetStepsPage,
};
use crate::StorageExecutor;

use crate::storage::storage_driver::{StorageDriver, StorageCmd};
use actix::Addr;

pub async fn set_steps_page<T: StorageDriver>(
    payload: Json<SetStepsPage>,
    storage: Data<Addr<StorageExecutor<T>>>
) -> Result<HttpResponse, Error> {
    debug!("new steps page request: {:?}", payload);
    let _result = storage
        .into_inner()
        .send(StorageCmd::UpdateStepsPage(payload.into_inner()))
        .await?;
    Ok(HttpResponse::Ok().await?)
}