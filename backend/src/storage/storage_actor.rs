use anyhow::Result;
// use crate::server::StorageDriver;
use proger_core::protocol::model::PageModel;
use crate::storage::storage_driver::{StorageDriver, StorageCmd};
use actix::{Actor, SyncContext, Message, Handler};
use log::{debug, info};

pub struct StorageExecutor<T: StorageDriver> {
    pub driver: T,
}

impl<T: StorageDriver> Actor for StorageExecutor<T> {
    type Context = SyncContext<Self>;
}

impl Message for StorageCmd {
    type Result = Result<PageModel>;
}

impl<T: StorageDriver> Handler<StorageCmd> for StorageExecutor<T> {
    type Result = Result<PageModel>;

    fn handle(&mut self, cmd: StorageCmd, _: &mut Self::Context) -> Self::Result {
        // Insert the session into the database
        println!("New command: {:?}", cmd);
        self.driver.write(cmd)?;
        Ok(PageModel{
            hashed_secret: "BANANA".to_string(),
            link: "BANANA".to_string(),
            steps: 0,
            start: 0,
            completed: 0,
            epoch_time: 0,
        })
    }
}
