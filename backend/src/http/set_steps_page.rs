use crate::StorageExecutor;
use actix_web::{
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use log::debug;
use proger_core::protocol::request::SetStepsPage;

use crate::storage::storage_driver::{StorageCmd, StorageDriver};
use actix::Addr;

pub async fn set_steps_page<T: StorageDriver>(
    link: Path<String>,
    payload: Json<SetStepsPage>,
    storage: Data<Addr<StorageExecutor<T>>>,
) -> Result<HttpResponse, Error> {
    debug!("new steps page request: {:?}", payload);
    let _result = storage
        .into_inner()
        .send(StorageCmd::UpdateStepsPage(
            link.to_string(),
            payload.into_inner(),
        ))
        .await?;
    Ok(HttpResponse::Ok().await?)
}
