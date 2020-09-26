use pkmnapi_db::types::MoveName;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type MoveNameResponse = BaseResponse<MoveNameResponseAttributes>;
pub type MoveNameResponseData = BaseResponseData<MoveNameResponseAttributes>;

impl MoveNameResponse {
    /// Create a new `MoveNameResponse`
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

#[derive(Debug, Serialize)]
pub struct MoveNameResponseAttributes {
    pub name: String,
}
