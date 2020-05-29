use crate::StorageExecutor;
use actix_web::{
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use log::debug;
use proger_core::protocol::request::DeleteStepsPage;

use crate::storage::storage_driver::{StorageCmd, StorageDriver};
use actix::Addr;

pub async fn delete_steps_page<T: StorageDriver>(
    link: Path<String>,
    payload: Json<DeleteStepsPage>,
    storage: Data<Addr<StorageExecutor<T>>>,
) -> Result<HttpResponse, Error> {
    debug!("new steps page request: {:?}", payload);
    let _result = storage
        .into_inner()
        .send(StorageCmd::DeleteStepsPage(
            link.to_string(),
            payload.into_inner(),
        ))
        .await?;
    Ok(HttpResponse::Ok().finish())
}
