use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput};
use proger_core::protocol::model::PageModel;
use crate::storage::storage_driver::{StorageDriver, StorageCmd};
use tokio::runtime::Runtime;
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct DynamoDbDriver(pub DynamoDbClient);

impl StorageDriver for DynamoDbDriver {
    fn connect(&self) -> Result<()> {
        Ok(())
    }

    fn write(&self, cmd: StorageCmd) -> Result<()> {
        println!("writing to db {:?}", cmd);
        match cmd {
            StorageCmd::CreateStepsPage(request) => {

                let epoch_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .as_secs();

                let model = PageModel {
                    hashed_secret: "HASHED_SECRET".to_string(),
                    link: "LINK".to_string(),
                    steps: request.steps,
                    start: request.start,
                    completed: request.start,
                    epoch_time,
                };
                
                let mut runtime = Runtime::new()?;
                println!("new runtime...");

                runtime.block_on(
                    self.0.put_item(PutItemInput {
                        table_name: "proger-pages".to_string(),
                        item: serde_dynamodb::to_hashmap(&model)?,
                        ..PutItemInput::default()
                    })
                );
                println!("done! WR");

                Ok(())
            },
            StorageCmd::UpdateStepsPage(request) => {
                Ok(())
            },

        }
    }
}