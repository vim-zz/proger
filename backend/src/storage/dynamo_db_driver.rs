use crate::storage::storage_driver::{self, StorageCmd, StorageDriver, StorageError};
use anyhow::{anyhow, Result};
use proger_core::protocol::model::PageModel;
use rusoto_dynamodb::{DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};
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

        let result = rt.block_on(self.0.get_item(GetItemInput {
            table_name: "proger-pages".to_string(),
            key: serde_dynamodb::to_hashmap(&key)?,
            ..GetItemInput::default()
        }))?;
        println!("READ: {:?}", result);

        if let Some(item) = result.item {
            Ok(serde_dynamodb::from_hashmap(item)?)
        } else {
            Err(anyhow!(StorageError::CorruptedItem))
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

    fn delete(&self, rt: &mut Runtime, link: String) -> Result<()> {
        let key = PageModelKey { link };

        let result = rt.block_on(self.0.delete_item(DeleteItemInput {
            table_name: "proger-pages".to_string(),
            key: serde_dynamodb::to_hashmap(&key)?,
            ..DeleteItemInput::default()
        }))?;
        println!("DELETE: {:?}", result);

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
                let password = storage_driver::generate_secret();
                let mut model = PageModel {
                    link: storage_driver::generate_link(),
                    secret: storage_driver::hash_secret(&password),
                    steps: request.steps,
                    start: request.start,
                    completed: request.start,
                    created: now,
                    updated: now,
                };

                self.write(rt, &model)?;

                // return the real password to the user
                model.secret = password;
                Ok(model)
            }

            StorageCmd::UpdateStepsPage(link, request) => {
                let mut model = self.read(rt, link)?;
                let hashed_input_password = &storage_driver::hash_secret(&request.admin_secret);
                let stored_hash = &model.secret;
                if stored_hash == hashed_input_password {
                    model.completed = request.completed;
                    model.updated = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                    self.write(rt, &model)?;
                    Ok(model)
                } else {
                    Err(anyhow!(StorageError::WrongPassword))
                }
            }

            StorageCmd::DeleteStepsPage(link, request) => {
                let model = self.read(rt, link.clone())?;
                let hashed_input_password = &storage_driver::hash_secret(&request.admin_secret);
                let stored_hash = &model.secret;
                if stored_hash == hashed_input_password {
                    self.delete(rt, link)?;
                    Ok(model)
                } else {
                    Err(anyhow!(StorageError::WrongPassword))
                }
            }

            StorageCmd::GetStepsPage(link) => self.read(rt, link),
        }
    }
}
