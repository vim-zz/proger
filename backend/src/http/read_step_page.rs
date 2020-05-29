use crate::StorageExecutor;
use actix_web::{
    web::{Data, Path},
    Error, HttpResponse,
};
use proger_core::protocol::response::StepPageProgress;

use crate::storage::storage_driver::{StorageCmd, StorageDriver};
use actix::Addr;

pub async fn read_step_page<T: StorageDriver>(
    link: Path<String>,
    storage: Data<Addr<StorageExecutor<T>>>,
) -> Result<HttpResponse, Error> {

    let result = storage
        .into_inner()
        .send(StorageCmd::ReadStepPage(link.to_string()))
        .await?;

    match result {
        Ok(page) => Ok(HttpResponse::Ok().json(StepPageProgress {
            steps: page.steps,
            completed: page.completed,
            updated: page.updated,
        })),
        Err(e) => Ok(HttpResponse::BadRequest().finish()),
    }
}
