//! Response specific implementations
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The logout response
pub struct PageID(pub String);