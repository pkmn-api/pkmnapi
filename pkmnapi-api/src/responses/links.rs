use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Links {
    #[serde(rename = "self")]
    pub _self: String,
}
