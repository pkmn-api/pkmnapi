use pkmnapi_db::TMName;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type TMNameResponse = BaseResponse<TMNameResponseAttributes>;
pub type TMNameResponseData = BaseResponseData<TMNameResponseAttributes>;
pub type TMNameResponseAll = BaseResponseAll<TMNameResponseData>;

impl TMNameResponseAll {
    pub fn new(tm_ids: &Vec<u8>, tm_names: &HashMap<u8, TMName>) -> TMNameResponseAll {
        TMNameResponseAll {
            data: tm_ids
                .iter()
                .map(|tm_id| TMNameResponseData::new(tm_id, tm_names.get(tm_id).unwrap()))
                .collect(),
            links: Links {
                _self: utils::generate_url("tms/names", None),
            },
        }
    }
}

impl TMNameResponse {
    pub fn new(tm_id: &u8, tm_name: &TMName) -> TMNameResponse {
        TMNameResponse {
            data: TMNameResponseData::new(tm_id, tm_name),
            links: Links {
                _self: utils::generate_url("tms/names", Some(&tm_id.to_string())),
            },
        }
    }
}

impl TMNameResponseData {
    pub fn new(tm_id: &u8, tm_name: &TMName) -> TMNameResponseData {
        BaseResponseData {
            id: tm_id.to_string(),
            _type: BaseResponseType::tm_names,
            attributes: TMNameResponseAttributes {
                name: tm_name.name.to_string(),
            },
            links: Links {
                _self: utils::generate_url("tms/names", Some(&tm_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TMNameResponseAttributes {
    pub name: String,
}
