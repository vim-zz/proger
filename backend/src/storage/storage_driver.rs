use anyhow::Result;
use proger_core::protocol::request::NewStepsPage;

/// The create session message
#[derive(Debug)]
pub enum StorageCmd {
    CreatePage(NewStepsPage),
}

/// Trait to allow different database backend
pub trait StorageDriver: 'static + Unpin {
    fn connect(&self) -> Result<()>;
    fn write(&self, cmd: StorageCmd) -> Result<()>;
}