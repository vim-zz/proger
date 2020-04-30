use rusoto_dynamodb::DynamoDbClient;
use crate::server::StorageDriver;
use anyhow::Result;

pub struct DynamoDbDriver {
    pub db: DynamoDbClient, 
}

impl StorageDriver for DynamoDbDriver {
    fn connect(&self) -> Result<()> {
        Ok(())
    }
}