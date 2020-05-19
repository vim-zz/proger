use anyhow::Result;
use proger_core::protocol::model::PageModel;
use proger_core::protocol::request::{NewStepsPage, SetStepsPage};
use tokio::runtime::Runtime;

/// The create session message
#[derive(Debug)]
pub enum StorageCmd {
    CreateStepsPage(NewStepsPage),
    UpdateStepsPage(String, SetStepsPage),
    GetStepsPage(String),
}

/// Trait to allow different database backend
pub trait StorageDriver: 'static + Unpin {
    fn connect(&self) -> Result<()>;
    fn execute(&self, rt: &mut Runtime, cmd: StorageCmd) -> Result<PageModel>;
}
