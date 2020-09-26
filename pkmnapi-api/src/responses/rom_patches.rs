use pkmnapi_sql::models::RomPatch;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type RomPatchResponse = BaseResponse<RomPatchResponseAttributes>;
pub type RomPatchResponseData = BaseResponseData<RomPatchResponseAttributes>;
pub type RomPatchesResponse = BaseResponseAll<RomPatchResponseData>;

impl RomPatchesResponse {
    /// Create a new `RomPatchesResponse`
    pub fn new(rom_patches: &Vec<RomPatch>) -> RomPatchesResponse {
        RomPatchesResponse {
            data: rom_patches
                .iter()
                .map(|rom_patch| RomPatchResponseData::new(rom_patch))
                .collect(),
            links: Links {
                _self: utils::generate_url("roms/patches", None),
            },
        }
    }
}

impl RomPatchResponse {
    /// Create a new `RomPatchResponse`
    pub fn new(rom_patch: &RomPatch) -> RomPatchResponse {
        RomPatchResponse {
            data: RomPatchResponseData::new(rom_patch),
            links: Links {
                _self: utils::generate_url("roms/patches", Some(&rom_patch.id)),
            },
        }
    }
}

impl RomPatchResponseData {
    pub fn new(rom_patch: &RomPatch) -> RomPatchResponseData {
        BaseResponseData {
            id: rom_patch.id.to_owned(),
            _type: BaseResponseType::rom_patches,
            attributes: RomPatchResponseAttributes {
                description: match &rom_patch.description {
                    Some(description) => Some(description.to_owned()),
                    None => None,
                },
            },
            links: Links {
                _self: utils::generate_url("roms/patches", Some(&rom_patch.id)),
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum RomPatchResponseType {
    rom_patches,
}

#[derive(Debug, Serialize)]
pub struct RomPatchResponseAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}
