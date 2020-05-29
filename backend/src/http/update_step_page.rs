use crate::StorageExecutor;
use actix_web::{
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use proger_core::protocol::request::UpdateStepPage;
use crate::storage::storage_driver::{StorageCmd, StorageDriver};
use actix::Addr;

pub async fn update_step_page<T: StorageDriver>(
    link: Path<String>,
    payload: Json<UpdateStepPage>,
    storage: Data<Addr<StorageExecutor<T>>>,
) -> Result<HttpResponse, Error> {
    let _result = storage
        .into_inner()
        .send(StorageCmd::UpdateStepPage(
            link.to_string(),
            payload.into_inner(),
        ))
        .await?;
    Ok(HttpResponse::Ok().finish())
}
