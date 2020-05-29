use crate::storage::storage_driver::{self, StorageCmd, StorageDriver, StorageError};
use anyhow::{anyhow, Result};
use proger_core::protocol::model::{StepPageModel, Step};
use rusoto_dynamodb::{DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use chrono::Utc;

/// Used to query dinamodb
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct StepPageModelKey {
    /// The page unique link, it is used as the index
    pub link: String,
}

#[derive(Clone)]
pub struct DynamoDbDriver(pub DynamoDbClient);

impl DynamoDbDriver {
    fn read(&self, rt: &mut Runtime, link: String) -> Result<StepPageModel> {
        let key = StepPageModelKey { link };

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

    fn write(&self, rt: &mut Runtime, model: &StepPageModel) -> Result<()> {
        let result = rt.block_on(self.0.put_item(PutItemInput {
            table_name: "proger-pages".to_string(),
            item: serde_dynamodb::to_hashmap(&model)?,
            ..PutItemInput::default()
        }))?;
        println!("WRITE: {:?}", result);

        Ok(())
    }

    fn delete(&self, rt: &mut Runtime, link: String) -> Result<()> {
        let key = StepPageModelKey { link };

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

    fn execute(&self, rt: &mut Runtime, cmd: StorageCmd) -> Result<StepPageModel> {
        println!("executing on db {:?}", cmd);
        match cmd {
            StorageCmd::CreateStepPage(request) => {
                let now = Utc::now();
                let password = storage_driver::generate_secret();
                let mut model = StepPageModel {
                    link: storage_driver::generate_link(),
                    secret: storage_driver::hash_secret(&password),
                    steps: request.steps,
                    completed: 0,
                    progress: vec![],
                    created: now,
                    updated: now,
                };

                self.write(rt, &model)?;

                // return the real password to the user
                model.secret = password;
                Ok(model)
            }

            StorageCmd::UpdateStepPage(link, request) => {
                let mut model = self.read(rt, link)?;
                let hashed_input_password = &storage_driver::hash_secret(&request.admin_secret);
                let stored_hash = &model.secret;
                let now = Utc::now();
                if stored_hash == hashed_input_password {
                    model.completed = request.step_completed;
                    model.updated = now;
                    model.progress.push(Step{
                        step: request.step_completed,
                        timestamp: now,
                    });
                    self.write(rt, &model)?;
                    Ok(model)
                } else {
                    Err(anyhow!(StorageError::WrongPassword))
                }
            }

            StorageCmd::DeleteStepPage(link, request) => {
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

            StorageCmd::ReadStepPage(link) => self.read(rt, link),
        }
    }
}
