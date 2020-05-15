use anyhow::Result;
use proger_core::protocol::request::{
    NewStepsPage,
    SetStepsPage,
};

/// The create session message
#[derive(Debug)]
pub enum StorageCmd {
    CreateStepsPage(NewStepsPage),
    UpdateStepsPage(SetStepsPage),
}

/// Trait to allow different database backend
pub trait StorageDriver: 'static + Unpin {
    fn connect(&self) -> Result<()>;
    fn write(&self, cmd: StorageCmd) -> Result<()>;
}

