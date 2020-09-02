use serde::Serialize;

use crate::responses::links::*;

#[derive(Serialize)]
pub struct Rom {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub attributes: RomAttributes,
    pub links: Links,
}

#[derive(Serialize)]
pub struct RomAttributes {
    pub name: String,
    pub hash: String,
    pub valid: bool,
}
