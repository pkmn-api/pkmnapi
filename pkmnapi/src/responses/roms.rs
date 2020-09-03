use serde::Serialize;

use crate::responses::links::Links;

#[derive(Serialize)]
pub struct RomResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub attributes: RomResponseAttributes,
    pub links: Links,
}

#[derive(Serialize)]
pub struct RomResponseAttributes {
    pub name: String,
    pub hash: String,
    pub valid: bool,
}
