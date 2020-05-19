use log::debug;

use crate::StorageExecutor;
use actix_web::{
    web::{Data, Path},
    Error, HttpResponse,
};

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
    // Ok(HttpResponse::Ok().json(PageAccess{
    //     admin_secret: "ADMIN_SECRET".to_string(),
    //     link: "PRIVATE_LINK".to_string(),
    // }))

    Ok(HttpResponse::Ok().finish())
}
