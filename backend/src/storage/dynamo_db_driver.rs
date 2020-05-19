use crate::storage::storage_driver::{StorageCmd, StorageDriver};
use anyhow::{anyhow, Result};
use proger_core::protocol::model::PageModel;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;

/// Used to query dinamodb
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct PageModelKey {
    /// The page unique link, it is used as the index
    pub link: String,
}

#[derive(Clone)]
pub struct DynamoDbDriver(pub DynamoDbClient);

impl DynamoDbDriver {
    fn read(&self, rt: &mut Runtime, link: String) -> Result<PageModel> {
        let key = PageModelKey { link };

        println!("reading {:?}", key);

        let result = rt.block_on(self.0.get_item(GetItemInput {
            table_name: "proger-pages".to_string(),
            key: serde_dynamodb::to_hashmap(&key)?,
            ..GetItemInput::default()
        }))?;
        println!("READ: {:?}", result);

        if let Some(item) = result.item {
            Ok(serde_dynamodb::from_hashmap(item)?)
        } else {
            Err(anyhow!("read empty item"))
        }
    }

    fn write(&self, rt: &mut Runtime, model: &PageModel) -> Result<()> {
        let result = rt.block_on(self.0.put_item(PutItemInput {
            table_name: "proger-pages".to_string(),
            item: serde_dynamodb::to_hashmap(&model)?,
            ..PutItemInput::default()
        }))?;
        println!("WRITE: {:?}", result);

        Ok(())
    }
}

impl StorageDriver for DynamoDbDriver {
    fn connect(&self) -> Result<()> {
        Ok(())
    }

    fn execute(&self, rt: &mut Runtime, cmd: StorageCmd) -> Result<PageModel> {
        println!("executing on db {:?}", cmd);
        match cmd {
            StorageCmd::CreateStepsPage(request) => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                let model = PageModel {
                    // TODO generate random unique link
                    link: "LINK".to_string(),
                    hashed_secret: "HASHED_SECRET".to_string(),
                    steps: request.steps,
                    start: request.start,
                    completed: request.start,
                    created: now,
                    updated: now,
                };

                self.write(rt, &model)?;

                Ok(model)
            }

            StorageCmd::UpdateStepsPage(link, request) => {
                let mut model = self.read(rt, link)?;
                model.completed = request.completed;
                model.updated = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                self.write(rt, &model)?;
                Ok(model)
            }

            StorageCmd::GetStepsPage(link) => self.read(rt, link),
        }
    }
}
