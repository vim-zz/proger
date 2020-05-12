use rusoto_dynamodb::DynamoDbClient;
use crate::storage::storage_driver::{StorageDriver, StorageCmd};
use anyhow::Result;

#[derive(Clone)]
pub struct DynamoDbDriver(pub DynamoDbClient);

impl StorageDriver for DynamoDbDriver {
    fn connect(&self) -> Result<()> {
        Ok(())
    }

    fn write(&self, cmd: StorageCmd) -> Result<()> {
        println!("writing to db {:?}", cmd);
        Ok(())
    }
}