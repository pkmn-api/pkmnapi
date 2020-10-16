use pkmnapi_db::MoveName;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type MoveNameResponse = BaseResponse<MoveNameResponseAttributes>;
pub type MoveNameResponseData = BaseResponseData<MoveNameResponseAttributes>;
pub type MoveNameResponseAll = BaseResponseAll<MoveNameResponseData>;

impl MoveNameResponseAll {
    pub fn new(move_ids: &Vec<u8>, move_names: &HashMap<u8, MoveName>) -> MoveNameResponseAll {
        MoveNameResponseAll {
            data: move_ids
                .iter()
                .map(|move_id| MoveNameResponseData::new(move_id, move_names.get(move_id).unwrap()))
                .collect(),
            links: Links {
                _self: utils::generate_url("moves/names", None),
            },
        }
    }
}

impl MoveNameResponse {
    pub fn new(move_id: &u8, move_name: &MoveName) -> MoveNameResponse {
        MoveNameResponse {
            data: MoveNameResponseData::new(move_id, move_name),
            links: Links {
                _self: utils::generate_url("moves/names", Some(&move_id.to_string())),
            },
        }
    }
}

impl MoveNameResponseData {
    pub fn new(move_id: &u8, move_name: &MoveName) -> MoveNameResponseData {
        BaseResponseData {
            id: move_id.to_string(),
            _type: BaseResponseType::move_names,
            attributes: MoveNameResponseAttributes {
                name: move_name.name.to_string(),
            },
            links: Links {
                _self: utils::generate_url("moves/names", Some(&move_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveNameResponseAttributes {
    pub name: String,
}
