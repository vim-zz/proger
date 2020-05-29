use log::debug;

use crate::StorageExecutor;
use actix_web::{
    web::{Data, Path},
    Error, HttpResponse,
};
use proger_core::protocol::response::Progress;

use crate::storage::storage_driver::{StorageCmd, StorageDriver};
use actix::Addr;

pub async fn view_page<T: StorageDriver>(
    link: Path<String>,
    storage: Data<Addr<StorageExecutor<T>>>,
) -> Result<HttpResponse, Error> {
    debug!("show link: {:?}", link);

    let result = storage
        .into_inner()
        .send(StorageCmd::GetStepsPage(link.to_string()))
        .await?;

    println!("result: {:?}", result);
    match result {
        Ok(page) => Ok(HttpResponse::Ok().json(Progress {
            steps: page.steps,
            start: page.start,
            completed: page.completed,
            updated: page.updated,
        })),
        Err(e) => Ok(HttpResponse::BadRequest().finish()),
    }
}
