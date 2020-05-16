use anyhow::Result;
// use crate::server::StorageDriver;
use proger_core::protocol::model::PageModel;
use crate::storage::storage_driver::{StorageDriver, StorageCmd};
use actix::{Actor, SyncContext, Message, Handler};
use log::{debug, info};
use tokio::runtime::Runtime;

pub struct StorageExecutor<T: StorageDriver> {
    pub driver: T,
    pub rt: Runtime,
}

impl<T: StorageDriver> Actor for StorageExecutor<T> {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
       println!("DB actor is up and running");
    }
}

impl Message for StorageCmd {
    type Result = Result<PageModel>;
}

impl<T: StorageDriver> Handler<StorageCmd> for StorageExecutor<T> {
    type Result = Result<PageModel>;

    fn handle(&mut self, cmd: StorageCmd, _: &mut Self::Context) -> Self::Result {
        // Insert the session into the database
        println!("New command: {:?}", cmd);
        let model = self.driver.execute(&mut self.rt, cmd)?;
        Ok(model)
    }
}
