use pkmnapi_sql::models::Rom;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::env;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type RomResponse = BaseResponse<RomResponseAttributes>;

impl RomResponse {
    /// Create a new `RomResponse`
    ///
    /// # Panics
    ///
    /// Panics if the `VALID_HASHES` environment variable is not set
    pub fn new(rom: &Rom) -> RomResponse {
        let valid_hashes = env::var("VALID_HASHES").expect("VALID_HASHES must be set");

        RomResponse {
            data: BaseResponseData {
                id: rom.id.to_owned(),
                _type: BaseResponseType::roms,
                attributes: RomResponseAttributes {
                    name: rom.name.to_owned(),
                    hash: rom.rom_data_id.to_owned(),
                    valid: valid_hashes.find(&rom.rom_data_id) != None,
                },
                links: Links {
                    _self: utils::generate_url("roms", None),
                },
            },
            links: Links {
                _self: utils::generate_url("roms", None),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RomResponseAttributes {
    pub name: String,
    pub hash: String,
    pub valid: bool,
}
