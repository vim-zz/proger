use anyhow::Result;
// use crate::server::StorageDriver;
use crate::storage::storage_driver::{StorageCmd, StorageDriver};
use actix::{Actor, Handler, Message, SyncContext};
use proger_core::protocol::model::StepPageModel;
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
    type Result = Result<StepPageModel>;
}

impl<T: StorageDriver> Handler<StorageCmd> for StorageExecutor<T> {
    type Result = Result<StepPageModel>;

    fn handle(&mut self, cmd: StorageCmd, _: &mut Self::Context) -> Self::Result {
        // Insert the session into the database
        println!("New command: {:?}", cmd);
        let model = self.driver.execute(&mut self.rt, cmd)?;
        Ok(model)
    }
}
