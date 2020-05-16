use log::debug;

use actix_web::{
    web::{
        Path,
        Data,
    },
    HttpResponse,
    Error,
};
use crate::StorageExecutor;

use crate::storage::storage_driver::{StorageDriver, StorageCmd};
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