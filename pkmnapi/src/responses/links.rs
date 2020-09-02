use serde::Serialize;

#[derive(Serialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub _self: String,
}
