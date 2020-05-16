use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, GetItemInput};
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

    fn execute(&self, rt: &mut Runtime, cmd: StorageCmd) -> Result<PageModel> {
        println!("writing to db {:?}", cmd);
        match cmd {
            StorageCmd::CreateStepsPage(request) => {

                let epoch_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .as_secs();

                let model = PageModel {
                    hashed_secret: "HASHED_SECRET".to_string(),
                    // TODO generate random unique link
                    link: "LINK".to_string(),
                    steps: request.steps,
                    start: request.start,
                    completed: request.start,
                    epoch_time,
                };
                println!("writing {:?}", model);
                
                let result = rt.block_on(
                    self.0.put_item(PutItemInput {
                        table_name: "proger-pages".to_string(),
                        item: serde_dynamodb::to_hashmap(&model)?,
                        ..PutItemInput::default()
                    })
                )?;
                println!("done! WR {:?}", result);

                Ok(model)
            },

            StorageCmd::UpdateStepsPage(request) => {
                todo!()
            },

            StorageCmd::GetStepsPage(link) => {
                let model = PageModel {
                    hashed_secret: "0".to_string(),
                    link,
                    steps: 0,
                    start: 0,
                    completed: 0,
                    epoch_time: 0,
                };

                println!("reading {:?}", model);
                
                let result = rt.block_on(
                    self.0.get_item(GetItemInput {
                        table_name: "proger-pages".to_string(),
                        key: serde_dynamodb::to_hashmap(&model)?,
                        ..GetItemInput::default()
                    })
                );
                println!("done! RD, {:?}", result);

                Ok(model)
            },

        }
    }
}