use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub _self: String,
}
