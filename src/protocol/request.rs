//! Request specific implementations
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The new page request
pub struct NewStepsPage {
    steps: u32,
    start: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Set  steps as done
pub struct SetStepsPage {
    completed: u32,
}