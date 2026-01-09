use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Command {
    Put(String, String),
    Get(String),
    Delete(String),
}
