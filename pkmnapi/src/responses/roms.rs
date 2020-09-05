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

#[derive(Debug, Serialize)]
pub struct RomResponseErrorNoRom {
    pub data: RomResponseErrorNoRomData,
}

impl RomResponseErrorNoRom {
    pub fn new() -> Self {
        RomResponseErrorNoRom {
            data: RomResponseErrorNoRomData {
                id: String::from("error_roms_no_rom"),
                _type: RomResponseErrorNoRomDataType::errors,
                attributes: RomResponseErrorNoRomDataAttributes {
                    message: String::from("No ROM uploaded"),
                },
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorNoRomData {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: RomResponseErrorNoRomDataType,
    pub attributes: RomResponseErrorNoRomDataAttributes,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum RomResponseErrorNoRomDataType {
    errors,
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorNoRomDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorInvalidRom {
    pub data: RomResponseErrorInvalidRomData,
}

impl RomResponseErrorInvalidRom {
    pub fn new() -> Self {
        RomResponseErrorInvalidRom {
            data: RomResponseErrorInvalidRomData {
                id: String::from("error_roms_invalid_rom"),
                _type: RomResponseErrorInvalidRomDataType::errors,
                attributes: RomResponseErrorInvalidRomDataAttributes {
                    message: String::from("Invalid ROM provided"),
                },
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorInvalidRomData {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: RomResponseErrorInvalidRomDataType,
    pub attributes: RomResponseErrorInvalidRomDataAttributes,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum RomResponseErrorInvalidRomDataType {
    errors,
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorInvalidRomDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorRomExists {
    pub data: RomResponseErrorRomExistsData,
}

impl RomResponseErrorRomExists {
    pub fn new() -> Self {
        RomResponseErrorRomExists {
            data: RomResponseErrorRomExistsData {
                id: String::from("error_roms_rom_exists"),
                _type: RomResponseErrorRomExistsDataType::errors,
                attributes: RomResponseErrorRomExistsDataAttributes {
                    message: String::from("ROM already exists"),
                },
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorRomExistsData {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: RomResponseErrorRomExistsDataType,
    pub attributes: RomResponseErrorRomExistsDataAttributes,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum RomResponseErrorRomExistsDataType {
    errors,
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorRomExistsDataAttributes {
    pub message: String,
}
