use crate::responses::links::Links;
use pkmnapi_sql::models::Rom;
use serde::Serialize;
use std::env;

#[derive(Debug, Serialize)]
pub struct RomResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: RomResponseType,
    pub attributes: RomResponseAttributes,
    pub links: Links,
}

impl RomResponse {
    /// Create a new `RomResponse`
    ///
    /// # Panics
    ///
    /// Panics if the `VALID_HASHES` environment variable is not set
    pub fn new(rom: &Rom) -> RomResponse {
        let valid_hashes = env::var("VALID_HASHES").expect("VALID_HASHES must be set");

        RomResponse {
            id: rom.id.to_owned(),
            _type: RomResponseType::roms,
            attributes: RomResponseAttributes {
                name: rom.name.to_owned(),
                hash: rom.rom_data_id.to_owned(),
                valid: valid_hashes.find(&rom.rom_data_id) != None,
            },
            links: Links {
                _self: "foo".to_string(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum RomResponseType {
    roms,
}

#[derive(Debug, Serialize)]
pub struct RomResponseAttributes {
    pub name: String,
    pub hash: String,
    pub valid: bool,
}
