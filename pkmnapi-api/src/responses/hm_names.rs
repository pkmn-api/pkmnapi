use pkmnapi_db::HMName;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type HMNameResponse = BaseResponse<HMNameResponseAttributes>;
pub type HMNameResponseData = BaseResponseData<HMNameResponseAttributes>;
pub type HMNameResponseAll = BaseResponseAll<HMNameResponseData>;

impl HMNameResponseAll {
    pub fn new(hm_ids: &Vec<u8>, hm_names: &HashMap<u8, HMName>) -> HMNameResponseAll {
        HMNameResponseAll {
            data: hm_ids
                .iter()
                .map(|hm_id| HMNameResponseData::new(hm_id, hm_names.get(hm_id).unwrap()))
                .collect(),
            links: Links {
                _self: utils::generate_url("hms/names", None),
            },
        }
    }
}

impl HMNameResponse {
    pub fn new(hm_id: &u8, hm_name: &HMName) -> HMNameResponse {
        HMNameResponse {
            data: HMNameResponseData::new(hm_id, hm_name),
            links: Links {
                _self: utils::generate_url("hms/names", Some(&hm_id.to_string())),
            },
        }
    }
}

impl HMNameResponseData {
    pub fn new(hm_id: &u8, hm_name: &HMName) -> HMNameResponseData {
        BaseResponseData {
            id: hm_id.to_string(),
            _type: BaseResponseType::hm_names,
            attributes: HMNameResponseAttributes {
                name: hm_name.name.to_string(),
            },
            links: Links {
                _self: utils::generate_url("hms/names", Some(&hm_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HMNameResponseAttributes {
    pub name: String,
}
