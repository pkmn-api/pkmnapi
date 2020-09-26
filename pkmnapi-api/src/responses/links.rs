use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub _self: String,
}
